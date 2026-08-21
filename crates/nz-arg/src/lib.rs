//! `nz-arg`：对齐 netwox toolarg 语义的参数描述与解析。
//!
//! 供 `nz` CLI、工具 0、未来 GUI Form 共用。不依赖 `nz-net`，不打印帮助，不删文件。

#![forbid(unsafe_code)]

mod argfile;
mod cmdline;
mod error;
mod parse;
mod schema;
mod value;

pub use error::ParseError;
pub use parse::{ParseMode, ParseOutcome, parse};
pub use schema::{ArgClass, ArgSchema, ArgSpec, ValueKind};
pub use value::{ArgValue, ParsedArgs};
