//! 网络地址子模块：IP / Ethernet / 端口与集合语法。
//!
//! 对照 `spec/netwib/net-addr.md` 与 netwib `net/{ip,eth,port,ips,eths,ports}.h`。

mod ethernet;
mod ip_set;
mod port_set;

pub use ethernet::{EthernetAddress, EthernetAddressSet};
pub use ip_set::IpAddressSet;
pub use port_set::PortSet;
