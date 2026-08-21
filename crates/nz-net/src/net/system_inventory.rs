//! 真系统本机配置（feature `system-inventory`）。
//!
//! 对照 `spec/netwib/net-conf.md`：Unix 上用 `if-addrs` 读网卡/IP；
//! Linux 再读 `/proc/net/{arp,route}`；其它 Unix 用接口前缀推导直连路由。
//! 不依赖 root；失败返回 [`Error::System`]。

use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr};

use if_addrs::{IfAddr, get_if_addrs};

use crate::error::{Error, Result};
use crate::net::conf::{
    ArpEntry, IpOnDevice, LocalConfiguration, Reachability, Route, RouteSource, lookup_arp,
    select_route,
};
use crate::net::device::{Device, DeviceInventory, HardwareType};

#[cfg(target_os = "linux")]
use crate::net::EthernetAddress;
#[cfg(target_os = "linux")]
use std::str::FromStr;

/// 一次查询得到的本机四表快照。
#[derive(Clone, Debug)]
pub struct SystemLocalConfiguration {
    devices: Vec<Device>,
    ip_addresses: Vec<IpOnDevice>,
    arp_entries: Vec<ArpEntry>,
    routes: Vec<Route>,
}

impl SystemLocalConfiguration {
    /// 读取本机网卡、IP、ARP、路由。
    ///
    /// # Errors
    ///
    /// 枚举接口失败，或平台路由/ARP 解析失败时返回 [`Error::System`]。
    pub fn query() -> Result<Self> {
        let interfaces = get_if_addrs().map_err(|error| Error::System {
            reason: format!("list interfaces: {error}"),
        })?;

        let mut names: Vec<String> = interfaces.iter().map(|iface| iface.name.clone()).collect();
        names.sort();
        names.dedup();

        let mut name_to_number: BTreeMap<String, u32> = BTreeMap::new();
        let mut devices = Vec::with_capacity(names.len());
        for (index, name) in names.iter().enumerate() {
            let number = u32::try_from(index + 1).unwrap_or(u32::MAX);
            name_to_number.insert(name.clone(), number);
            let is_loopback = interfaces
                .iter()
                .any(|iface| &iface.name == name && iface.is_loopback());
            let hardware_type = if is_loopback {
                HardwareType::Loopback
            } else {
                HardwareType::Ethernet
            };
            devices.push(Device {
                number,
                easy_name: easy_name_for(name, hardware_type),
                real_name: name.clone(),
                hardware_type,
                mtu: if is_loopback { 65536 } else { 1500 },
                ethernet_address: None,
            });
        }

        let mut ip_addresses = Vec::new();
        for iface in &interfaces {
            let Some(&device_number) = name_to_number.get(&iface.name) else {
                continue;
            };
            let (address, netmask) = match &iface.addr {
                IfAddr::V4(v4) => (IpAddr::V4(v4.ip), IpAddr::V4(v4.netmask)),
                IfAddr::V6(v6) => (IpAddr::V6(v6.ip), IpAddr::V6(v6.netmask)),
            };
            ip_addresses.push(IpOnDevice {
                device_number,
                address,
                netmask,
                is_ppp: false,
                ppp_peer: None,
            });
        }

        let (arp_entries, routes) = {
            #[cfg(target_os = "linux")]
            {
                (
                    parse_proc_arp(&name_to_number)?,
                    parse_proc_route(&name_to_number, &ip_addresses)?,
                )
            }
            #[cfg(not(target_os = "linux"))]
            {
                let _ = &name_to_number;
                (Vec::new(), derived_on_link_routes(&ip_addresses))
            }
        };

        Ok(Self {
            devices,
            ip_addresses,
            arp_entries,
            routes,
        })
    }
}

impl DeviceInventory for SystemLocalConfiguration {
    fn list_devices(&self) -> Result<Vec<Device>> {
        Ok(self.devices.clone())
    }
}

impl LocalConfiguration for SystemLocalConfiguration {
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
            .iter()
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

fn easy_name_for(real_name: &str, hardware_type: HardwareType) -> String {
    match hardware_type {
        HardwareType::Loopback => String::from("Lo0"),
        HardwareType::Ethernet => {
            let digits: String = real_name
                .chars()
                .rev()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
                .chars()
                .rev()
                .collect();
            if digits.is_empty() {
                format!("Eth-{real_name}")
            } else {
                format!("Eth{digits}")
            }
        }
        _ => real_name.to_owned(),
    }
}

#[cfg(any(test, not(target_os = "linux")))]
fn derived_on_link_routes(ip_addresses: &[IpOnDevice]) -> Vec<Route> {
    ip_addresses
        .iter()
        .filter_map(|entry| {
            let destination = network_address(entry.address, entry.netmask)?;
            Some(Route {
                device_number: entry.device_number,
                destination,
                netmask: entry.netmask,
                source: RouteSource::Address(entry.address),
                gateway: None,
                metric: 0,
            })
        })
        .collect()
}

#[cfg(any(test, not(target_os = "linux")))]
fn network_address(address: IpAddr, netmask: IpAddr) -> Option<IpAddr> {
    match (address, netmask) {
        (IpAddr::V4(addr), IpAddr::V4(mask)) => Some(IpAddr::V4(Ipv4Addr::from(
            u32::from(addr) & u32::from(mask),
        ))),
        (IpAddr::V6(addr), IpAddr::V6(mask)) => Some(IpAddr::V6(std::net::Ipv6Addr::from(
            u128::from(addr) & u128::from(mask),
        ))),
        _ => None,
    }
}

#[cfg(target_os = "linux")]
fn parse_proc_arp(name_to_number: &BTreeMap<String, u32>) -> Result<Vec<ArpEntry>> {
    let text = std::fs::read_to_string("/proc/net/arp").map_err(|error| Error::System {
        reason: format!("read /proc/net/arp: {error}"),
    })?;
    let mut entries = Vec::new();
    for line in text.lines().skip(1) {
        let columns: Vec<&str> = line.split_whitespace().collect();
        if columns.len() < 6 {
            continue;
        }
        let Ok(ip) = IpAddr::from_str(columns[0]) else {
            continue;
        };
        let Ok(ethernet) = EthernetAddress::from_str(columns[3]) else {
            continue;
        };
        let Some(&device_number) = name_to_number.get(columns[5]) else {
            continue;
        };
        entries.push(ArpEntry {
            device_number,
            ethernet,
            ip,
        });
    }
    Ok(entries)
}

#[cfg(target_os = "linux")]
fn parse_proc_route(
    name_to_number: &BTreeMap<String, u32>,
    ip_addresses: &[IpOnDevice],
) -> Result<Vec<Route>> {
    let text = std::fs::read_to_string("/proc/net/route").map_err(|error| Error::System {
        reason: format!("read /proc/net/route: {error}"),
    })?;
    let mut routes = Vec::new();
    for line in text.lines().skip(1) {
        let columns: Vec<&str> = line.split_whitespace().collect();
        if columns.len() < 8 {
            continue;
        }
        let Some(&device_number) = name_to_number.get(columns[0]) else {
            continue;
        };
        let Ok(destination_bits) = u32::from_str_radix(columns[1], 16) else {
            continue;
        };
        let Ok(gateway_bits) = u32::from_str_radix(columns[2], 16) else {
            continue;
        };
        let Ok(mask_bits) = u32::from_str_radix(columns[7], 16) else {
            continue;
        };
        let metric = columns
            .get(6)
            .and_then(|value| value.parse().ok())
            .unwrap_or(0);
        let destination = IpAddr::V4(Ipv4Addr::from(destination_bits.swap_bytes()));
        let netmask = IpAddr::V4(Ipv4Addr::from(mask_bits.swap_bytes()));
        let gateway = if gateway_bits == 0 {
            None
        } else {
            Some(IpAddr::V4(Ipv4Addr::from(gateway_bits.swap_bytes())))
        };
        let source = ip_addresses
            .iter()
            .find(|entry| entry.device_number == device_number && entry.address.is_ipv4())
            .map_or(RouteSource::Local, |entry| {
                RouteSource::Address(entry.address)
            });
        routes.push(Route {
            device_number,
            destination,
            netmask,
            source,
            gateway,
            metric,
        });
    }
    Ok(routes)
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use super::{SystemLocalConfiguration, derived_on_link_routes, easy_name_for, network_address};
    use crate::net::EthernetAddress;
    use crate::net::conf::{ArpEntry, IpOnDevice, LocalConfiguration, Route, RouteSource};
    use crate::net::device::{Device, DeviceInventory, HardwareType};

    /// spec `conf_real_ignored_in_ci` / 真路径可列举（不依赖特定网卡名）
    #[test]
    fn system_configuration_lists_without_root() {
        let configuration = SystemLocalConfiguration::query().expect("query");
        let devices = configuration.list_devices().expect("devices");
        let ips = configuration.list_ip_addresses().expect("ips");
        assert!(
            !devices.is_empty(),
            "expected at least loopback on CI hosts"
        );
        assert!(!ips.is_empty(), "expected at least one address");
        assert!(configuration.list_arp_entries().is_ok());
        let routes = configuration.list_routes().expect("routes");
        assert!(!routes.is_empty());
        assert!(devices.iter().any(|device| {
            device.hardware_type == HardwareType::Loopback && device.easy_name == "Lo0"
        }));
    }

    #[test]
    fn easy_name_and_network_helpers() {
        assert_eq!(easy_name_for("lo0", HardwareType::Loopback), "Lo0");
        assert_eq!(easy_name_for("en0", HardwareType::Ethernet), "Eth0");
        assert_eq!(
            easy_name_for("bridge", HardwareType::Ethernet),
            "Eth-bridge"
        );
        assert_eq!(easy_name_for("ppp0", HardwareType::Ppp), "ppp0");

        let v4 = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10));
        let mask_v4 = IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0));
        assert_eq!(
            network_address(v4, mask_v4),
            Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 0)))
        );
        let v6 = IpAddr::V6(Ipv6Addr::LOCALHOST);
        let mask_v6 = IpAddr::V6(Ipv6Addr::from(0xffff_ffff_ffff_ffff_u128 << 64));
        assert!(network_address(v6, mask_v6).is_some());
        assert_eq!(network_address(v4, mask_v6), None);

        let routes = derived_on_link_routes(&[IpOnDevice {
            device_number: 1,
            address: v4,
            netmask: mask_v4,
            is_ppp: false,
            ppp_peer: None,
        }]);
        assert_eq!(routes.len(), 1);
        assert_eq!(
            routes[0].destination,
            IpAddr::V4(Ipv4Addr::new(192, 168, 1, 0))
        );
    }

    #[test]
    fn system_reach_covers_gateway_and_local_source() {
        let configuration = SystemLocalConfiguration {
            devices: vec![Device {
                number: 1,
                easy_name: String::from("Eth0"),
                real_name: String::from("en0"),
                hardware_type: HardwareType::Ethernet,
                mtu: 1500,
                ethernet_address: Some(EthernetAddress::from_bytes([1, 2, 3, 4, 5, 6])),
            }],
            ip_addresses: vec![IpOnDevice {
                device_number: 1,
                address: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
                netmask: IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)),
                is_ppp: false,
                ppp_peer: None,
            }],
            arp_entries: vec![ArpEntry {
                device_number: 1,
                ethernet: EthernetAddress::from_bytes([9, 9, 9, 9, 9, 1]),
                ip: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            }],
            routes: vec![
                Route {
                    device_number: 1,
                    destination: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)),
                    netmask: IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0)),
                    source: RouteSource::Local,
                    gateway: None,
                    metric: 0,
                },
                Route {
                    device_number: 1,
                    destination: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                    netmask: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                    source: RouteSource::Address(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2))),
                    gateway: Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))),
                    metric: 1,
                },
            ],
        };

        let local = configuration
            .reach(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 9)))
            .expect("local");
        assert_eq!(local.device_number, Some(1));
        assert_eq!(
            local.source_ip,
            Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)))
        );
        assert!(!local.unresolved);

        let via_gw = configuration
            .reach(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)))
            .expect("gw");
        assert_eq!(
            via_gw.destination_ethernet,
            Some(EthernetAddress::from_bytes([9, 9, 9, 9, 9, 1]))
        );

        let missing = configuration
            .reach(IpAddr::V6(Ipv6Addr::LOCALHOST))
            .expect("missing");
        assert!(missing.unresolved);
    }
}
