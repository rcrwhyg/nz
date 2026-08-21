//! IPv4 头编解码（含自动 ihl / totlen / checksum）。

use std::net::Ipv4Addr;

use crate::dat::checksum;
use crate::error::{Error, Result};

/// IPv4 协议号常用值。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Ipv4Protocol {
    /// ICMP。
    Icmp,
    /// TCP。
    Tcp,
    /// UDP。
    Udp,
    /// 其它协议号。
    Other(u8),
}

impl Ipv4Protocol {
    /// 协议号字节。
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Icmp => 1,
            Self::Tcp => 6,
            Self::Udp => 17,
            Self::Other(value) => value,
        }
    }

    /// 从协议号解析。
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Icmp,
            6 => Self::Tcp,
            17 => Self::Udp,
            other => Self::Other(other),
        }
    }
}

/// IPv4 头字段。
///
/// `internet_header_length` / `total_length` / `header_checksum` / `protocol` 为 `None` 时组包自动填。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ipv4Packet {
    /// TOS。
    pub type_of_service: u8,
    /// 标识。
    pub identification: u16,
    /// DF 标志。
    pub dont_fragment: bool,
    /// MF 标志。
    pub more_fragments: bool,
    /// 片偏移（以 8 字节为单位）。
    pub fragment_offset: u16,
    /// TTL。
    pub time_to_live: u8,
    /// 协议；`None` 由上层推断。
    pub protocol: Option<Ipv4Protocol>,
    /// 头校验和；`None` 自动计算，`Some` 字面写入。
    pub header_checksum: Option<u16>,
    /// 源地址。
    pub source: Ipv4Addr,
    /// 目的地址。
    pub destination: Ipv4Addr,
    /// 选项（长度须 4 字节对齐）。
    pub options: Vec<u8>,
    /// 总长度；`None` 自动。
    pub total_length: Option<u16>,
    /// IHL（32 位字）；`None` 自动。
    pub internet_header_length: Option<u8>,
}

impl Ipv4Packet {
    /// 解析后的协议（缺省 UDP 仅作占位，调用方应检查）。
    #[must_use]
    pub fn resolved_protocol(&self) -> Ipv4Protocol {
        self.protocol.unwrap_or(Ipv4Protocol::Udp)
    }

    /// 若未设置则返回默认值，否则返回已设协议。
    #[must_use]
    pub fn protocol_or_default(&self, default: Ipv4Protocol) -> Ipv4Protocol {
        self.protocol.unwrap_or(default)
    }

    /// 编码头 + 载荷。
    ///
    /// # Errors
    ///
    /// 选项未对齐、长度溢出或协议缺失时返回错误。
    pub fn encode_with_payload(&self, payload: &[u8]) -> Result<Vec<u8>> {
        if !self.options.len().is_multiple_of(4) {
            return Err(Error::invalid_parameter(
                "ipv4 options must be 4-byte aligned",
            ));
        }
        let header_length = 20 + self.options.len();
        if header_length > 60 {
            return Err(Error::invalid_parameter("ipv4 header too large"));
        }
        let ihl = self
            .internet_header_length
            .unwrap_or(u8::try_from(header_length / 4).unwrap_or(5));
        let protocol = self
            .protocol
            .ok_or_else(|| Error::invalid_parameter("ipv4 protocol required for encode"))?;
        let total_length = self.total_length.unwrap_or(
            u16::try_from(header_length + payload.len())
                .map_err(|_| Error::invalid_parameter("ipv4 total length overflow"))?,
        );

        let mut header = vec![0u8; header_length];
        header[0] = 0x40 | (ihl & 0x0F);
        header[1] = self.type_of_service;
        header[2..4].copy_from_slice(&total_length.to_be_bytes());
        header[4..6].copy_from_slice(&self.identification.to_be_bytes());
        let mut flags_offset = self.fragment_offset & 0x1FFF;
        if self.dont_fragment {
            flags_offset |= 1 << 14;
        }
        if self.more_fragments {
            flags_offset |= 1 << 13;
        }
        header[6..8].copy_from_slice(&flags_offset.to_be_bytes());
        header[8] = self.time_to_live;
        header[9] = protocol.as_u8();
        header[12..16].copy_from_slice(&self.source.octets());
        header[16..20].copy_from_slice(&self.destination.octets());
        if !self.options.is_empty() {
            header[20..].copy_from_slice(&self.options);
        }

        let header_checksum = self.header_checksum.unwrap_or_else(|| checksum(&header));
        header[10..12].copy_from_slice(&header_checksum.to_be_bytes());

        let mut bytes = header;
        bytes.extend_from_slice(payload);
        Ok(bytes)
    }

    /// 解码头，返回头与载荷。
    ///
    /// # Errors
    ///
    /// 截断或 IHL 非法时返回错误。
    pub fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        if bytes.len() < 20 {
            return Err(Error::invalid_parameter("ipv4 header truncated"));
        }
        let version_ihl = bytes[0];
        if version_ihl >> 4 != 4 {
            return Err(Error::invalid_parameter("not ipv4"));
        }
        let ihl = version_ihl & 0x0F;
        let header_length = usize::from(ihl) * 4;
        if header_length < 20 || bytes.len() < header_length {
            return Err(Error::invalid_parameter("ipv4 ihl invalid"));
        }
        let total_length = u16::from_be_bytes([bytes[2], bytes[3]]);
        let identification = u16::from_be_bytes([bytes[4], bytes[5]]);
        let flags_offset = u16::from_be_bytes([bytes[6], bytes[7]]);
        let header_checksum = u16::from_be_bytes([bytes[10], bytes[11]]);
        let options = bytes[20..header_length].to_vec();
        let payload_end = usize::from(total_length).min(bytes.len());
        if payload_end < header_length {
            return Err(Error::invalid_parameter(
                "ipv4 total length shorter than header",
            ));
        }
        Ok((
            Self {
                type_of_service: bytes[1],
                identification,
                dont_fragment: (flags_offset & (1 << 14)) != 0,
                more_fragments: (flags_offset & (1 << 13)) != 0,
                fragment_offset: flags_offset & 0x1FFF,
                time_to_live: bytes[8],
                protocol: Some(Ipv4Protocol::from_u8(bytes[9])),
                header_checksum: Some(header_checksum),
                source: Ipv4Addr::new(bytes[12], bytes[13], bytes[14], bytes[15]),
                destination: Ipv4Addr::new(bytes[16], bytes[17], bytes[18], bytes[19]),
                options,
                total_length: Some(total_length),
                internet_header_length: Some(ihl),
            },
            &bytes[header_length..payload_end],
        ))
    }
}
