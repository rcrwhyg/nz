//! UDP 数据报编解码（含 IPv4 伪头校验和）。

use std::net::Ipv4Addr;

use crate::dat::InternetChecksum;
use crate::error::{Error, Result};
use crate::pkt::ipv4::Ipv4Protocol;

/// UDP 头与载荷。
///
/// `length` / `checksum` 为 `None` 时组包自动计算。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdpDatagram {
    /// 源端口。
    pub source_port: u16,
    /// 目的端口。
    pub destination_port: u16,
    /// 长度（头+载荷）；`None` 自动。
    pub length: Option<u16>,
    /// 校验和；`None` 自动（含伪头），`Some` 字面写入。
    pub checksum: Option<u16>,
    /// 载荷。
    pub payload: Vec<u8>,
}

impl UdpDatagram {
    /// 使用 IPv4 伪头编码。
    ///
    /// # Errors
    ///
    /// 长度溢出时返回参数错误。
    pub fn encode_with_ipv4_pseudo(
        &self,
        source: Ipv4Addr,
        destination: Ipv4Addr,
        protocol: Ipv4Protocol,
    ) -> Result<Vec<u8>> {
        let length = self.length.unwrap_or(
            u16::try_from(8 + self.payload.len())
                .map_err(|_| Error::invalid_parameter("udp length overflow"))?,
        );
        let mut bytes = Vec::with_capacity(usize::from(length));
        bytes.extend_from_slice(&self.source_port.to_be_bytes());
        bytes.extend_from_slice(&self.destination_port.to_be_bytes());
        bytes.extend_from_slice(&length.to_be_bytes());
        bytes.extend_from_slice(&0u16.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        // 截断或填充到声明长度
        bytes.resize(usize::from(length), 0);

        let checksum_value = match self.checksum {
            Some(value) => value,
            None => ipv4_udp_checksum(source, destination, protocol, &bytes),
        };
        bytes[6..8].copy_from_slice(&checksum_value.to_be_bytes());
        Ok(bytes)
    }

    /// 解码 UDP（不校验伪头和，保留线上海校验和字段）。
    ///
    /// # Errors
    ///
    /// 缓冲短于 8 字节或 length 非法时返回错误。
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(Error::invalid_parameter("udp header truncated"));
        }
        let source_port = u16::from_be_bytes([bytes[0], bytes[1]]);
        let destination_port = u16::from_be_bytes([bytes[2], bytes[3]]);
        let length = u16::from_be_bytes([bytes[4], bytes[5]]);
        let checksum_value = u16::from_be_bytes([bytes[6], bytes[7]]);
        if usize::from(length) < 8 || usize::from(length) > bytes.len() {
            return Err(Error::invalid_parameter("udp length invalid"));
        }
        Ok(Self {
            source_port,
            destination_port,
            length: Some(length),
            checksum: Some(checksum_value),
            payload: bytes[8..usize::from(length)].to_vec(),
        })
    }
}

fn ipv4_udp_checksum(
    source: Ipv4Addr,
    destination: Ipv4Addr,
    protocol: Ipv4Protocol,
    udp_bytes: &[u8],
) -> u16 {
    let mut hasher = InternetChecksum::new();
    hasher.update(&source.octets());
    hasher.update(&destination.octets());
    hasher.update(&[0, protocol.as_u8()]);
    let udp_length = u16::try_from(udp_bytes.len()).unwrap_or(u16::MAX);
    hasher.update(&udp_length.to_be_bytes());
    hasher.update(udp_bytes);
    let mut value = hasher.finish();
    // RFC 768：算出 0 时改为 0xFFFF
    if value == 0 {
        value = 0xFFFF;
    }
    value
}
