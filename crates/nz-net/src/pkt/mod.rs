//! 分层报文编解码（Ethernet / IPv4 / UDP 最小集）。
//!
//! 对照 `spec/netwib/pkt.md` 与 `nz-packet-codec`：未指定高级字段则自动计算；
//! 指定则字面写入。本任务只交付 Eth+IPv4+UDP roundtrip；TCP/ARP/ICMP/conv/ipfrag 后续分期。

mod ethernet;
mod ipv4;
mod udp;

pub use ethernet::{EthernetFrame, EthernetType};
pub use ipv4::{Ipv4Packet, Ipv4Protocol};
pub use udp::UdpDatagram;

use crate::error::{Error, Result};
use crate::net::EthernetAddress;
use std::net::Ipv4Addr;

/// 最小链路栈：Ethernet + IPv4 + UDP。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthernetIpv4Udp {
    /// 以太网头字段。
    pub ethernet: EthernetFrame,
    /// IPv4 头字段。
    pub ipv4: Ipv4Packet,
    /// UDP 头与载荷。
    pub udp: UdpDatagram,
}

impl EthernetIpv4Udp {
    /// 编码为线字节；自动填以太网 type、IPv4 ihl/totlen/protocol/checksum、UDP len/checksum。
    ///
    /// # Errors
    ///
    /// 选项过长或字段非法时返回 [`Error::InvalidParameter`]。
    pub fn encode(&self) -> Result<Vec<u8>> {
        let udp_bytes = self.udp.encode_with_ipv4_pseudo(
            self.ipv4.source,
            self.ipv4.destination,
            self.ipv4.protocol_or_default(Ipv4Protocol::Udp),
        )?;
        let mut ipv4 = self.ipv4.clone();
        if ipv4.protocol.is_none() {
            ipv4.protocol = Some(Ipv4Protocol::Udp);
        }
        let ip_bytes = ipv4.encode_with_payload(&udp_bytes)?;
        let mut ethernet = self.ethernet.clone();
        if ethernet.ether_type.is_none() {
            ethernet.ether_type = Some(EthernetType::Ipv4);
        }
        ethernet.encode_with_payload(&ip_bytes)
    }

    /// 从线字节解码 Ethernet + IPv4 + UDP。
    ///
    /// # Errors
    ///
    /// 长度不足、类型不匹配或校验字段非法时返回错误。
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let (ethernet, after_eth) = EthernetFrame::decode(bytes)?;
        if ethernet.resolved_ether_type() != EthernetType::Ipv4 {
            return Err(Error::invalid_parameter("expected ethernet type IPv4"));
        }
        let (ipv4, after_ip) = Ipv4Packet::decode(after_eth)?;
        if ipv4.resolved_protocol() != Ipv4Protocol::Udp {
            return Err(Error::invalid_parameter("expected IPv4 protocol UDP"));
        }
        let udp = UdpDatagram::decode(after_ip)?;
        Ok(Self {
            ethernet,
            ipv4,
            udp,
        })
    }
}

/// 构造最小测试用 Eth+IPv4+UDP（自动字段均未指定）。
#[must_use]
pub fn sample_ethernet_ipv4_udp() -> EthernetIpv4Udp {
    EthernetIpv4Udp {
        ethernet: EthernetFrame {
            destination: EthernetAddress::from_bytes([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]),
            source: EthernetAddress::from_bytes([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]),
            ether_type: None,
        },
        ipv4: Ipv4Packet {
            type_of_service: 0,
            identification: 0x1234,
            dont_fragment: false,
            more_fragments: false,
            fragment_offset: 0,
            time_to_live: 64,
            protocol: None,
            header_checksum: None,
            source: Ipv4Addr::new(192, 168, 1, 10),
            destination: Ipv4Addr::new(192, 168, 1, 20),
            options: Vec::new(),
            total_length: None,
            internet_header_length: None,
        },
        udp: UdpDatagram {
            source_port: 12345,
            destination_port: 53,
            length: None,
            checksum: None,
            payload: b"hello".to_vec(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{EthernetIpv4Udp, sample_ethernet_ipv4_udp};
    use crate::pkt::ipv4::Ipv4Packet;

    /// spec `eth_ip4_udp_roundtrip`
    #[test]
    fn eth_ip4_udp_roundtrip() {
        let packet = sample_ethernet_ipv4_udp();
        let encoded = packet.encode().expect("encode");
        let decoded = EthernetIpv4Udp::decode(&encoded).expect("decode");
        let reencoded = decoded.encode().expect("reencode");
        assert_eq!(encoded, reencoded);
        assert_eq!(decoded.udp.payload, b"hello");
        assert_eq!(decoded.ipv4.source, packet.ipv4.source);
        assert_eq!(decoded.udp.source_port, 12345);
    }

    /// spec `ipv4_literal_checksum_not_recomputed`
    #[test]
    fn ipv4_literal_checksum_not_recomputed() {
        let mut packet = sample_ethernet_ipv4_udp();
        packet.ipv4.header_checksum = Some(0xABCD);
        let encoded = packet.encode().expect("encode");
        // Ethernet(14) + IPv4 checksum at offset 10..12 within IP header → absolute 24..26
        assert_eq!(&encoded[24..26], &0xABCDu16.to_be_bytes());
        let (header, _) = Ipv4Packet::decode(&encoded[14..]).expect("ip decode");
        assert_eq!(header.header_checksum, Some(0xABCD));
    }

    #[test]
    fn decode_rejects_short_frames_and_wrong_types() {
        assert!(EthernetIpv4Udp::decode(&[0u8; 10]).is_err());
        let mut packet = sample_ethernet_ipv4_udp();
        packet.ethernet.ether_type = Some(crate::pkt::EthernetType::Arp);
        let encoded = packet.encode().expect("encode arp typed");
        // Force ARP type on wire then try stack decode
        assert!(EthernetIpv4Udp::decode(&encoded).is_err());
    }

    #[test]
    fn udp_literal_checksum_preserved() {
        let mut packet = sample_ethernet_ipv4_udp();
        packet.udp.checksum = Some(0x1111);
        let encoded = packet.encode().expect("encode");
        let decoded = EthernetIpv4Udp::decode(&encoded).expect("decode");
        assert_eq!(decoded.udp.checksum, Some(0x1111));
    }

    #[test]
    fn ipv4_options_and_flags_roundtrip() {
        use crate::pkt::Ipv4Protocol;
        use crate::pkt::ipv4::Ipv4Packet;
        use std::net::Ipv4Addr;

        let packet = Ipv4Packet {
            type_of_service: 0,
            identification: 1,
            dont_fragment: true,
            more_fragments: true,
            fragment_offset: 8,
            time_to_live: 32,
            protocol: Some(Ipv4Protocol::Udp),
            header_checksum: None,
            source: Ipv4Addr::new(1, 2, 3, 4),
            destination: Ipv4Addr::new(5, 6, 7, 8),
            options: vec![0x01, 0x00, 0x00, 0x00],
            total_length: None,
            internet_header_length: None,
        };
        let encoded = packet.encode_with_payload(b"xy").expect("encode");
        let (decoded, payload) = Ipv4Packet::decode(&encoded).expect("decode");
        assert!(decoded.dont_fragment);
        assert!(decoded.more_fragments);
        assert_eq!(decoded.fragment_offset, 8);
        assert_eq!(payload, b"xy");
        assert!(
            Ipv4Packet {
                options: vec![1, 2, 3],
                ..packet
            }
            .encode_with_payload(&[])
            .is_err()
        );
    }

    #[test]
    fn ethernet_type_helpers_and_udp_errors() {
        use crate::net::EthernetAddress;
        use crate::pkt::ethernet::{EthernetFrame, EthernetType};
        use crate::pkt::udp::UdpDatagram;
        use std::net::Ipv4Addr;

        assert_eq!(EthernetType::from_u16(0x0800), EthernetType::Ipv4);
        assert_eq!(EthernetType::from_u16(0x86DD), EthernetType::Ipv6);
        assert_eq!(EthernetType::Other(0x1234).as_u16(), 0x1234);
        assert!(
            EthernetFrame {
                destination: EthernetAddress::from_bytes([0; 6]),
                source: EthernetAddress::from_bytes([0; 6]),
                ether_type: None,
            }
            .encode_with_payload(&[])
            .is_err()
        );
        assert!(UdpDatagram::decode(&[0u8; 4]).is_err());
        let datagram = UdpDatagram {
            source_port: 1,
            destination_port: 2,
            length: None,
            checksum: None,
            payload: Vec::new(),
        };
        let encoded = datagram
            .encode_with_ipv4_pseudo(
                Ipv4Addr::new(1, 1, 1, 1),
                Ipv4Addr::new(2, 2, 2, 2),
                crate::pkt::Ipv4Protocol::Udp,
            )
            .expect("udp");
        assert_eq!(encoded.len(), 8);
    }
}
