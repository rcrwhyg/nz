//! 可变字节缓冲。
//!
//! 对照 netwib `dat/buf.h` 的读写语义；不暴露 C 窗指针，内部用 [`Vec<u8>`]。

/// 可读写的字节缓冲。
///
/// 写入后再读出应得到相同字节序列（spec `buf_roundtrip_bytes`）。
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ByteBuffer {
    bytes: Vec<u8>,
}

impl ByteBuffer {
    /// 创建空缓冲。
    #[must_use]
    pub fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    /// 从已有字节向量创建缓冲。
    #[must_use]
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// 当前内容长度（字节）。
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// 缓冲是否为空。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// 只读视图。
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// 追加单字节。
    pub fn push_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    /// 追加字节切片。
    pub fn extend_from_slice(&mut self, slice: &[u8]) {
        self.bytes.extend_from_slice(slice);
    }

    /// 清空内容，保留已分配容量。
    pub fn clear(&mut self) {
        self.bytes.clear();
    }

    /// 取出内部字节向量。
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::ByteBuffer;

    /// spec `buf_roundtrip_bytes`
    #[test]
    fn buf_roundtrip_bytes() {
        let mut buffer = ByteBuffer::new();
        buffer.extend_from_slice(&[0x01, 0x02, 0xFF]);
        buffer.push_byte(0x00);
        assert_eq!(buffer.as_slice(), &[0x01, 0x02, 0xFF, 0x00]);
        assert_eq!(buffer.len(), 4);
        assert!(!buffer.is_empty());
        buffer.clear();
        assert!(buffer.is_empty());
        assert_eq!(buffer.into_bytes(), Vec::<u8>::new());
    }

    #[test]
    fn buffer_from_bytes_constructor() {
        let buffer = ByteBuffer::from_bytes(vec![0xAA, 0xBB]);
        assert_eq!(buffer.as_slice(), &[0xAA, 0xBB]);
    }
}
