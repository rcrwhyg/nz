//! `dat` 子集：缓冲、编解码、Internet 校验和。
//!
//! 对照 `spec/netwib/dat.md` 与 netwib `dat/{buf,bufenc,bufdec,checksum}.h`。
//! 不引入额外运行时依赖（Base64 等为自研实现）。

mod buffer;
mod checksum;
mod decode;
mod encode;

pub use buffer::ByteBuffer;
pub use checksum::{InternetChecksum, checksum};
pub use decode::{DecodeFormat, decode_input};
pub use encode::{EncodeFormat, encode_bytes};
