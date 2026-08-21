//! 网络地址与本机配置。
//!
//! 对照 `spec/netwib/net-addr.md`、`net-device.md`、`net-conf.md`。

mod conf;
mod device;
mod ethernet;
mod ip_set;
mod port_set;

pub use conf::{
    ArpEntry, FakeLocalConfiguration, IpOnDevice, LocalConfiguration, Reachability, Route,
    RouteSource,
};
pub use device::{Device, DeviceInventory, FakeDeviceInventory, HardwareType};
pub use ethernet::{EthernetAddress, EthernetAddressSet};
pub use ip_set::IpAddressSet;
pub use port_set::PortSet;

#[cfg(feature = "system-inventory")]
pub use conf::list_system_configuration_stub;
#[cfg(feature = "system-inventory")]
pub use device::list_system_devices;
