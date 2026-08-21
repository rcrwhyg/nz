//! 本机网络配置：IP / ARP / 路由 / 到达。
//!
//! 对照 `spec/netwib/net-conf.md` 与 netwib `net/{conf,confip,confarp,confrout}.h`。
//! 默认测试走 [`FakeLocalConfiguration`]；真读取在 `system-inventory` feature 下启用。

use std::net::{IpAddr, Ipv4Addr};

use crate::error::Result;
use crate::net::EthernetAddress;
use crate::net::device::{Device, DeviceInventory, FakeDeviceInventory};

/// 网卡上的一条 IP 配置。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IpOnDevice {
    /// 所属网卡编号。
    pub device_number: u32,
    /// 地址。
    pub address: IpAddr,
    /// 掩码（IPv4 为点分掩码语义；IPv6 用前缀长度编码进同字段时由调用方约定）。
    pub netmask: IpAddr,
    /// 是否 PPP。
    pub is_ppp: bool,
    /// PPP 对端；非 PPP 时为 `None`。
    pub ppp_peer: Option<IpAddr>,
}

/// ARP / neighbor 表项（IPv4 与 IPv6 同一表）。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArpEntry {
    /// 网卡编号。
    pub device_number: u32,
    /// Ethernet 地址。
    pub ethernet: EthernetAddress,
    /// IP。
    pub ip: IpAddr,
}

/// 路由源：具体地址或本机（`local`）。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RouteSource {
    /// 源地址为 `local`（对照工具 1 展示）。
    Local,
    /// 显式源 IP。
    Address(IpAddr),
}

/// 一条路由。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Route {
    /// 出接口编号。
    pub device_number: u32,
    /// 目的网络。
    pub destination: IpAddr,
    /// 掩码。
    pub netmask: IpAddr,
    /// 源。
    pub source: RouteSource,
    /// 网关；直连为本机网段时为 `None`。
    pub gateway: Option<IpAddr>,
    /// 度量。
    pub metric: u32,
}

/// 到达解析结果（工具 6 / 工具 0 `--conf` 字段）。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reachability {
    /// 出接口编号；未解析时为 `None`。
    pub device_number: Option<u32>,
    /// 源 IP；未解析时为 `None`。
    pub source_ip: Option<IpAddr>,
    /// 源 MAC；非以太或未解析时为 `None`。
    pub source_ethernet: Option<EthernetAddress>,
    /// 目的 MAC（本网段为对端；跨网段为网关）；未解析时为 `None`。
    pub destination_ethernet: Option<EthernetAddress>,
    /// 无匹配路由或字段缺失时为 `true`（不崩溃）。
    pub unresolved: bool,
}

impl Reachability {
    /// 构造「未解析」结果。
    #[must_use]
    pub fn unresolved() -> Self {
        Self {
            device_number: None,
            source_ip: None,
            source_ethernet: None,
            destination_ethernet: None,
            unresolved: true,
        }
    }
}

/// 本机配置后端。
pub trait LocalConfiguration: DeviceInventory {
    /// 列出 IP 配置。
    ///
    /// # Errors
    ///
    /// 后端失败时返回错误。
    fn list_ip_addresses(&self) -> Result<Vec<IpOnDevice>>;

    /// 列出 ARP / neighbor。
    ///
    /// # Errors
    ///
    /// 后端失败时返回错误。
    fn list_arp_entries(&self) -> Result<Vec<ArpEntry>>;

    /// 列出路由。
    ///
    /// # Errors
    ///
    /// 后端失败时返回错误。
    fn list_routes(&self) -> Result<Vec<Route>>;

    /// 计算到达 `destination` 的出接口与地址字段。
    ///
    /// # Errors
    ///
    /// 后端失败时返回错误；无路由时返回 `Ok` 且 [`Reachability::unresolved`]。
    fn reach(&self, destination: IpAddr) -> Result<Reachability>;
}

/// 测试用假配置表。
#[derive(Clone, Debug)]
pub struct FakeLocalConfiguration {
    devices: FakeDeviceInventory,
    ip_addresses: Vec<IpOnDevice>,
    arp_entries: Vec<ArpEntry>,
    routes: Vec<Route>,
}

impl FakeLocalConfiguration {
    /// 样例四表齐全（spec `conf_fake_lists_four_tables`）。
    #[must_use]
    pub fn sample() -> Self {
        let devices = FakeDeviceInventory::sample();
        let gateway_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
        let host_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10));
        let neighbor_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 20));
        Self {
            devices,
            ip_addresses: vec![IpOnDevice {
                device_number: 1,
                address: host_ip,
                netmask: IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)),
                is_ppp: false,
                ppp_peer: None,
            }],
            arp_entries: vec![
                ArpEntry {
                    device_number: 1,
                    ethernet: EthernetAddress::from_bytes([0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0x01]),
                    ip: host_ip,
                },
                ArpEntry {
                    device_number: 1,
                    ethernet: EthernetAddress::from_bytes([0x11, 0x22, 0x33, 0x44, 0x55, 0x01]),
                    ip: gateway_ip,
                },
                ArpEntry {
                    device_number: 1,
                    ethernet: EthernetAddress::from_bytes([0x11, 0x22, 0x33, 0x44, 0x55, 0x20]),
                    ip: neighbor_ip,
                },
            ],
            routes: vec![
                Route {
                    device_number: 1,
                    destination: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 0)),
                    netmask: IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)),
                    source: RouteSource::Address(host_ip),
                    gateway: None,
                    metric: 0,
                },
                Route {
                    device_number: 1,
                    destination: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                    netmask: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                    source: RouteSource::Address(host_ip),
                    gateway: Some(gateway_ip),
                    metric: 1,
                },
            ],
        }
    }
}

impl DeviceInventory for FakeLocalConfiguration {
    fn list_devices(&self) -> Result<Vec<Device>> {
        self.devices.list_devices()
    }
}

impl LocalConfiguration for FakeLocalConfiguration {
    fn list_ip_addresses(&self) -> Result<Vec<IpOnDevice>> {
        Ok(self.ip_addresses.clone())
    }

    fn list_arp_entries(&self) -> Result<Vec<ArpEntry>> {
        Ok(self.arp_entries.clone())
    }

    fn list_routes(&self) -> Result<Vec<Route>> {
        Ok(self.routes.clone())
    }

    fn reach(&self, destination: IpAddr) -> Result<Reachability> {
        let Some(route) = select_route(&self.routes, destination) else {
            return Ok(Reachability::unresolved());
        };

        let source_ip = match route.source {
            RouteSource::Address(address) => Some(address),
            RouteSource::Local => self
                .ip_addresses
                .iter()
                .find(|entry| entry.device_number == route.device_number)
                .map(|entry| entry.address),
        };

        let source_ethernet = self
            .devices
            .list_devices()?
            .into_iter()
            .find(|device| device.number == route.device_number)
            .and_then(|device| device.ethernet_address);

        let destination_ethernet = if let Some(gateway) = route.gateway {
            lookup_arp(&self.arp_entries, route.device_number, gateway)
        } else {
            lookup_arp(&self.arp_entries, route.device_number, destination)
        };

        let unresolved = source_ip.is_none();
        Ok(Reachability {
            device_number: Some(route.device_number),
            source_ip,
            source_ethernet,
            destination_ethernet,
            unresolved,
        })
    }
}

/// 按最长前缀匹配选择路由。
pub(crate) fn select_route(routes: &[Route], destination: IpAddr) -> Option<&Route> {
    routes
        .iter()
        .filter(|route| route_matches(route, destination))
        .max_by_key(|route| prefix_length(route.netmask))
}

fn route_matches(route: &Route, destination: IpAddr) -> bool {
    match (route.destination, route.netmask, destination) {
        (IpAddr::V4(network), IpAddr::V4(mask), IpAddr::V4(target)) => {
            let network_bits = u32::from(network);
            let mask_bits = u32::from(mask);
            let target_bits = u32::from(target);
            (target_bits & mask_bits) == (network_bits & mask_bits)
        }
        (IpAddr::V6(network), IpAddr::V6(mask), IpAddr::V6(target)) => {
            let network_bits = u128::from(network);
            let mask_bits = u128::from(mask);
            let target_bits = u128::from(target);
            (target_bits & mask_bits) == (network_bits & mask_bits)
        }
        _ => false,
    }
}

fn prefix_length(netmask: IpAddr) -> u32 {
    match netmask {
        IpAddr::V4(mask) => u32::from(mask).count_ones(),
        IpAddr::V6(mask) => u128::from(mask).count_ones(),
    }
}

/// 在 ARP 表中查找 `device_number` + `ip` 对应的 Ethernet 地址。
pub(crate) fn lookup_arp(
    entries: &[ArpEntry],
    device_number: u32,
    ip: IpAddr,
) -> Option<EthernetAddress> {
    entries
        .iter()
        .find(|entry| entry.device_number == device_number && entry.ip == ip)
        .map(|entry| entry.ethernet)
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use super::{DeviceInventory, FakeLocalConfiguration, LocalConfiguration, Reachability};
    use crate::net::device::HardwareType;

    /// spec `conf_fake_lists_four_tables`
    #[test]
    fn conf_fake_lists_four_tables() {
        let configuration = FakeLocalConfiguration::sample();
        let devices = configuration.list_devices().expect("devices");
        let ips = configuration.list_ip_addresses().expect("ips");
        let arp = configuration.list_arp_entries().expect("arp");
        let routes = configuration.list_routes().expect("routes");

        assert!(
            devices
                .iter()
                .any(|device| device.hardware_type == HardwareType::Ethernet)
        );
        assert!(!ips.is_empty());
        assert!(!arp.is_empty());
        assert!(!routes.is_empty());
        assert!(ips[0].address.is_ipv4());
        assert!(arp[0].ethernet.to_string().contains(':'));
        assert_eq!(routes[0].device_number, 1);
    }

    /// spec `conf_reach_local_subnet`
    #[test]
    fn conf_reach_local_subnet() {
        let configuration = FakeLocalConfiguration::sample();
        let reach = configuration
            .reach(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 20)))
            .expect("reach");
        assert!(!reach.unresolved);
        assert_eq!(reach.device_number, Some(1));
        assert_eq!(
            reach.source_ip,
            Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)))
        );
        assert_eq!(
            reach
                .destination_ethernet
                .map(|address| address.to_string()),
            Some(String::from("11:22:33:44:55:20"))
        );
    }

    /// spec `conf_reach_via_gw_sets_dst_eth`
    #[test]
    fn conf_reach_via_gw_sets_dst_eth() {
        let configuration = FakeLocalConfiguration::sample();
        let reach = configuration
            .reach(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)))
            .expect("reach");
        assert!(!reach.unresolved);
        assert_eq!(
            reach
                .destination_ethernet
                .map(|address| address.to_string()),
            Some(String::from("11:22:33:44:55:01"))
        );
    }

    /// spec `conf_reach_missing_is_unresolved`
    #[test]
    fn conf_reach_missing_is_unresolved() {
        let configuration = FakeLocalConfiguration {
            devices: crate::net::device::FakeDeviceInventory::empty(),
            ip_addresses: Vec::new(),
            arp_entries: Vec::new(),
            routes: Vec::new(),
        };
        let reach = configuration
            .reach(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)))
            .expect("reach");
        assert_eq!(reach, Reachability::unresolved());
        assert!(reach.unresolved);
    }

    /// spec `conf_real_ignored_in_ci`
    #[test]
    fn conf_real_ignored_in_ci() {
        assert!(FakeLocalConfiguration::sample().list_routes().is_ok());
        #[cfg(feature = "system-inventory")]
        {
            assert!(crate::net::SystemLocalConfiguration::query().is_ok());
        }
    }
}
