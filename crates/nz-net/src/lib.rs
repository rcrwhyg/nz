//! 网络协议库：对齐 netwib **能力**，不复制 C API。
//!
//! 依赖方向：本 crate → `nz` CLI → `nz-gui`。工具特例不得绕过本库编解码。

pub mod error;

pub use error::{Error, ErrorPartition, Result};
