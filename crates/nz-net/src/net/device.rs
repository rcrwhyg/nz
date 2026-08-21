//! 本机网卡列举。
//!
//! 对照 `spec/netwib/net-device.md` 与 netwib `net/{device,confdev}.h`。
//! 默认测试走 [`FakeDeviceInventory`]；真网卡读取仅在 `system-inventory` feature 下提供桩。

use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::net::EthernetAddress;

/// 网卡硬件类型（对照 `device.h` hwtype）。
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HardwareType {
    /// 未知或未分类。
    Unknown,
    /// 以太网（含 Wi-Fi；对照源未单独列无线）。
    Ethernet,
    /// 环回。
    Loopback,
    /// PPP。
    Ppp,
    /// 过时 parallel。
    Parallel,
    /// 过时 serial。
    Serial,
}

impl HardwareType {
    /// 稳定小写名，供展示与解析。
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Ethernet => "ether",
            Self::Loopback => "loopback",
            Self::Ppp => "ppp",
            Self::Parallel => "parallel",
            Self::Serial => "serial",
        }
    }
}

impl fmt::Display for HardwareType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for HardwareType {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self> {
        match text {
            "unknown" => Ok(Self::Unknown),
            "ether" | "ethernet" => Ok(Self::Ethernet),
            "loopback" | "lo" => Ok(Self::Loopback),
            "ppp" => Ok(Self::Ppp),
            "parallel" => Ok(Self::Parallel),
            "serial" => Ok(Self::Serial),
            _ => Err(Error::invalid_parameter("unknown hardware type")),
        }
    }
}

/// 一块本机网卡（列举字段，不含 sniff/spoof DLT）。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Device {
    /// 编号（对照 conf 表索引）。
    pub number: u32,
    /// 易记名（原 `deviceeasy`，如 `Eth0`）。
    pub easy_name: String,
    /// 真实设备名（如 `en0`、`lo0`）。
    pub real_name: String,
    /// 硬件类型。
    pub hardware_type: HardwareType,
    /// MTU。
    pub mtu: u32,
    /// Ethernet 地址；仅 `hardware_type == Ethernet` 时通常有值。
    pub ethernet_address: Option<EthernetAddress>,
}

/// 网卡列举后端。
pub trait DeviceInventory {
    /// 列出本机网卡。
    ///
    /// # Errors
    ///
    /// 后端不可用或读取失败时返回错误。
    fn list_devices(&self) -> Result<Vec<Device>>;
}

/// 测试用假网卡表。
#[derive(Clone, Debug, Default)]
pub struct FakeDeviceInventory {
    devices: Vec<Device>,
}

impl FakeDeviceInventory {
    /// 空表。
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// 样例：一块 ether + 一块 loopback（spec `device_list_fake`）。
    ///
    /// # Panics
    ///
    /// 不会在正常常量地址下 panic；样例 MAC 为硬编码合法字面量。
    #[must_use]
    pub fn sample() -> Self {
        Self {
            devices: vec![
                Device {
                    number: 1,
                    easy_name: String::from("Eth0"),
                    real_name: String::from("en0"),
                    hardware_type: HardwareType::Ethernet,
                    mtu: 1500,
                    ethernet_address: Some(EthernetAddress::from_bytes([
                        0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0x01,
                    ])),
                },
                Device {
                    number: 2,
                    easy_name: String::from("Lo0"),
                    real_name: String::from("lo0"),
                    hardware_type: HardwareType::Loopback,
                    mtu: 16384,
                    ethernet_address: None,
                },
            ],
        }
    }

    /// 用给定列表构造。
    #[must_use]
    pub fn from_devices(devices: Vec<Device>) -> Self {
        Self { devices }
    }
}

impl DeviceInventory for FakeDeviceInventory {
    fn list_devices(&self) -> Result<Vec<Device>> {
        Ok(self.devices.clone())
    }
}

/// 真系统网卡列举。
///
/// 当前闸仅提供 feature 门控桩，返回未实现；CI 默认不启用，不依赖 root。
///
/// # Errors
///
/// 始终返回 [`Error::NotImplemented`]（直至后续接入平台后端）。
#[cfg(feature = "system-inventory")]
pub fn list_system_devices() -> Result<Vec<Device>> {
    Err(Error::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::{DeviceInventory, FakeDeviceInventory, HardwareType};

    /// spec `device_hwtype_roundtrip`
    #[test]
    fn device_hwtype_roundtrip() {
        let types = [
            HardwareType::Unknown,
            HardwareType::Ethernet,
            HardwareType::Loopback,
            HardwareType::Ppp,
            HardwareType::Parallel,
            HardwareType::Serial,
        ];
        for hardware_type in types {
            let text = hardware_type.to_string();
            assert!(!text.is_empty());
            assert_eq!(text.parse::<HardwareType>().expect("parse"), hardware_type);
        }
        assert_eq!(
            "ethernet".parse::<HardwareType>().expect("alias"),
            HardwareType::Ethernet
        );
        assert!("nope".parse::<HardwareType>().is_err());
    }

    /// spec `device_list_fake`
    #[test]
    fn device_list_fake() {
        let devices = FakeDeviceInventory::sample()
            .list_devices()
            .expect("fake list");
        assert!(
            devices
                .iter()
                .any(|device| device.hardware_type == HardwareType::Ethernet
                    && device.ethernet_address.is_some()
                    && !device.easy_name.is_empty()
                    && !device.real_name.is_empty()
                    && device.mtu > 0)
        );
        assert!(
            devices
                .iter()
                .any(|device| device.hardware_type == HardwareType::Loopback
                    && device.ethernet_address.is_none())
        );
    }

    /// spec `device_list_real_ignored_in_ci`
    #[test]
    fn device_list_real_ignored_in_ci() {
        // 假后端始终可用；真路径（feature）返回未实现，不碰网卡/root。
        assert!(FakeDeviceInventory::empty().list_devices().is_ok());
        #[cfg(feature = "system-inventory")]
        {
            assert!(matches!(
                super::list_system_devices(),
                Err(crate::Error::NotImplemented)
            ));
        }
    }
}
