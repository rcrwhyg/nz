//! Ethernet II 帧头编解码。

use crate::error::{Error, Result};
use crate::net::EthernetAddress;

/// 以太网类型字段常用值。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EthernetType {
    /// IPv4（0x0800）。
    Ipv4,
    /// ARP（0x0806）。
    Arp,
    /// IPv6（0x86DD）。
    Ipv6,
    /// 其它类型字面值。
    Other(u16),
}

impl EthernetType {
    /// 线序类型值。
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        match self {
            Self::Ipv4 => 0x0800,
            Self::Arp => 0x0806,
            Self::Ipv6 => 0x86DD,
            Self::Other(value) => value,
        }
    }

    /// 从类型字解析。
    #[must_use]
    pub const fn from_u16(value: u16) -> Self {
        match value {
            0x0800 => Self::Ipv4,
            0x0806 => Self::Arp,
            0x86DD => Self::Ipv6,
            other => Self::Other(other),
        }
    }
}

/// Ethernet II 头（不含 802.1Q）。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthernetFrame {
    /// 目的 MAC。
    pub destination: EthernetAddress,
    /// 源 MAC。
    pub source: EthernetAddress,
    /// `EtherType`；`None` 表示组包时由上层推断。
    pub ether_type: Option<EthernetType>,
}

impl EthernetFrame {
    /// 解析后得到的类型（缺省按 IPv4 仅用于内部推断错误路径外）。
    #[must_use]
    pub fn resolved_ether_type(&self) -> EthernetType {
        self.ether_type.unwrap_or(EthernetType::Ipv4)
    }

    /// 编码头 + 载荷。
    ///
    /// # Errors
    ///
    /// `ether_type` 未指定时返回参数错误。
    pub fn encode_with_payload(&self, payload: &[u8]) -> Result<Vec<u8>> {
        let ether_type = self
            .ether_type
            .ok_or_else(|| Error::invalid_parameter("ethernet type required for encode"))?;
        let mut bytes = Vec::with_capacity(14 + payload.len());
        bytes.extend_from_slice(self.destination.octets());
        bytes.extend_from_slice(self.source.octets());
        bytes.extend_from_slice(&ether_type.as_u16().to_be_bytes());
        bytes.extend_from_slice(payload);
        Ok(bytes)
    }

    /// 解码头，返回头与剩余载荷。
    ///
    /// # Errors
    ///
    /// 缓冲短于 14 字节时返回参数错误。
    pub fn decode(bytes: &[u8]) -> Result<(Self, &[u8])> {
        if bytes.len() < 14 {
            return Err(Error::invalid_parameter("ethernet frame truncated"));
        }
        let destination = EthernetAddress::from_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5],
        ]);
        let source = EthernetAddress::from_bytes([
            bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11],
        ]);
        let type_value = u16::from_be_bytes([bytes[12], bytes[13]]);
        Ok((
            Self {
                destination,
                source,
                ether_type: Some(EthernetType::from_u16(type_value)),
            },
            &bytes[14..],
        ))
    }
}
