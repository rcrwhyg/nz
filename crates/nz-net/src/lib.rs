//! 网络协议库：对齐 netwib **能力**，不复制 C API。
//!
//! 依赖方向：本 crate → `nz` CLI → `nz-gui`。工具特例不得绕过本库编解码。

pub mod dat;
pub mod error;
pub mod net;
pub mod pkt;

pub use dat::{
    ByteBuffer, DecodeFormat, EncodeFormat, InternetChecksum, checksum, decode_input, encode_bytes,
};
pub use error::{Error, ErrorPartition, Result};
pub use net::{
    ArpEntry, Device, DeviceInventory, EthernetAddress, EthernetAddressSet, FakeDeviceInventory,
    FakeLocalConfiguration, HardwareType, IpAddressSet, IpOnDevice, LocalConfiguration, PortSet,
    Reachability, Route, RouteSource,
};

#[cfg(feature = "system-inventory")]
pub use net::{SystemLocalConfiguration, list_system_devices};
pub use pkt::{
    EthernetFrame, EthernetIpv4Udp, EthernetType, Ipv4Packet, Ipv4Protocol, UdpDatagram,
    sample_ethernet_ipv4_udp,
};
