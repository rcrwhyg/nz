//! TCP/IP Internet 校验和（一补码）。
//!
//! 对照 `netwib/dat/checksum.h`：结果为**主机序** [`u16`]；空缓冲为 `0xFFFF`。
//! 支持整块计算与分段 [`InternetChecksum`] update/finish，供 IP/TCP/UDP 伪头使用。

/// 对整块数据计算 Internet 校验和。
///
/// 空切片返回 `0xFFFF`（spec `checksum_empty_is_ffff`）。
#[must_use]
pub fn checksum(data: &[u8]) -> u16 {
    let mut hasher = InternetChecksum::new();
    hasher.update(data);
    hasher.finish()
}

/// 分段累加 Internet 校验和。
#[derive(Clone, Debug, Default)]
pub struct InternetChecksum {
    sum: u32,
    /// 奇数字节挂起（等待与下一字节组成 16 位字）。
    pending_byte: Option<u8>,
}

impl InternetChecksum {
    /// 创建空累加器。
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 追加数据；可多次调用。
    pub fn update(&mut self, data: &[u8]) {
        let mut index = 0usize;

        if let Some(high) = self.pending_byte.take() {
            if index < data.len() {
                self.add_word(high, data[index]);
                index += 1;
            } else {
                self.pending_byte = Some(high);
                return;
            }
        }

        while index + 1 < data.len() {
            self.add_word(data[index], data[index + 1]);
            index += 2;
        }

        if index < data.len() {
            self.pending_byte = Some(data[index]);
        }
    }

    /// 完成计算并返回主机序校验和。
    #[must_use]
    pub fn finish(mut self) -> u16 {
        if let Some(high) = self.pending_byte.take() {
            self.add_word(high, 0);
        }
        fold_to_ones_complement(self.sum)
    }

    fn add_word(&mut self, high: u8, low: u8) {
        self.sum += u32::from(u16::from_be_bytes([high, low]));
    }
}

fn fold_to_ones_complement(mut sum: u32) -> u16 {
    while sum > 0xFFFF {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    // 折叠后 sum 必在 u16 范围内。
    (!sum & 0xFFFF) as u16
}

#[cfg(test)]
mod tests {
    use super::{InternetChecksum, checksum};

    /// spec `checksum_empty_is_ffff`
    #[test]
    fn checksum_empty_is_ffff() {
        assert_eq!(checksum(&[]), 0xFFFF);
        assert_eq!(InternetChecksum::new().finish(), 0xFFFF);
    }

    /// spec `checksum_incremental_matches_oneshot`
    #[test]
    fn checksum_incremental_matches_oneshot() {
        let data = [0x00, 0x01, 0x02, 0x03, 0x04];
        let whole = checksum(&data);

        let mut hasher = InternetChecksum::new();
        hasher.update(&data[..2]);
        hasher.update(&data[2..]);
        assert_eq!(hasher.finish(), whole);

        let mut split = InternetChecksum::new();
        split.update(&data[..1]);
        split.update(&data[1..4]);
        split.update(&data[4..]);
        assert_eq!(split.finish(), whole);
    }

    #[test]
    fn checksum_known_vector() {
        // RFC 1071 附录 A："123456789" → 0xF62A（主机序一补码）
        assert_eq!(checksum(b"123456789"), 0xF62A);
    }

    #[test]
    fn checksum_pending_byte_with_empty_update() {
        let mut hasher = InternetChecksum::new();
        hasher.update(&[0x01]);
        hasher.update(&[]);
        hasher.update(&[0x02]);
        assert_eq!(hasher.finish(), checksum(&[0x01, 0x02]));
    }
}
