//! 统一失败分类，供库与 CLI 展示。
//!
//! 对照 `netwib-src/src/netwib/err.h` 的**分区语义**（数据/参数/逻辑/系统/用户），
//! 不把 C 数值表当作稳定 ABI。工具 0 `--error` 打印本模块类型的可读文本。
//! 若某工具输出依赖具体码，在该工具 spec 写明。

use thiserror::Error;

/// 库与 CLI 共用的 `Result` 别名。
pub type Result<T> = std::result::Result<T, Error>;

/// 错误语义分区。
///
/// 数值区间仅作对照说明，实现不保证与 netwib 整数 1:1。
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ErrorPartition {
    /// 常见数据或路由类失败（对照 1000–1999）。
    DataOrRoute,
    /// 参数错误（对照 2000–2999）。
    Parameter,
    /// 逻辑错误：未实现、对象未初始化（对照 3000–3999）。
    Logic,
    /// 函数或系统错误（对照 4000–9999）。
    System,
    /// 调用方自定义（对照 ≥10000）。
    UserDefined,
}

/// `nz-net` 失败类型。
///
/// 变体表达分区与常见原因；`Display` 给人类可读串。禁止 `netwib_*` 符号。
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
    /// 缓冲或流已读尽，没有更多数据。
    #[error("end of data")]
    DataEnded,

    /// 当前资源或条件不可用（例如设备、路由信息暂时缺失）。
    #[error("not available")]
    NotAvailable,

    /// 找不到到达目标的路由。
    #[error("route not found")]
    RouteNotFound,

    /// 参数非法（范围、编码、对齐等）。
    #[error("invalid parameter: {reason}")]
    InvalidParameter {
        /// 人类可读的失败原因。
        reason: String,
    },

    /// 请求的能力尚未实现。
    #[error("not implemented")]
    NotImplemented,

    /// 对象尚未初始化就调用了依赖初始化的操作。
    #[error("not initialized")]
    NotInitialized,

    /// 底层函数或操作系统失败。
    #[error("system error: {reason}")]
    System {
        /// 人类可读的失败原因。
        reason: String,
    },

    /// 用户自定义错误，与库变体分属不同分区，不会互相覆盖。
    #[error("user-defined error {numeric_hint}: {message}")]
    UserDefined {
        /// 调用方自选的提示码（对照 netwib ≥10000；不要求等于 C 码）。
        numeric_hint: u32,
        /// 人类可读说明。
        message: String,
    },
}

impl Error {
    /// 构造参数错误。
    pub fn invalid_parameter(reason: impl Into<String>) -> Self {
        Self::InvalidParameter {
            reason: reason.into(),
        }
    }

    /// 构造系统错误。
    pub fn system(reason: impl Into<String>) -> Self {
        Self::System {
            reason: reason.into(),
        }
    }

    /// 构造用户分区错误。
    ///
    /// `numeric_hint` 只作展示与调用方映射，不与库变体的判别式冲突。
    pub fn user_defined(numeric_hint: u32, message: impl Into<String>) -> Self {
        Self::UserDefined {
            numeric_hint,
            message: message.into(),
        }
    }

    /// 返回本错误所属语义分区。
    #[must_use]
    pub fn partition(&self) -> ErrorPartition {
        match self {
            Self::DataEnded | Self::NotAvailable | Self::RouteNotFound => {
                ErrorPartition::DataOrRoute
            }
            Self::InvalidParameter { .. } => ErrorPartition::Parameter,
            Self::NotImplemented | Self::NotInitialized => ErrorPartition::Logic,
            Self::System { .. } => ErrorPartition::System,
            Self::UserDefined { .. } => ErrorPartition::UserDefined,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, ErrorPartition, Result};

    /// spec `error_ok_is_success`：成功路径不是错误。
    #[test]
    fn error_ok_is_success() {
        let outcome: Result<()> = Ok(());
        assert!(outcome.is_ok());
        assert!(outcome.as_ref().ok().is_some());
    }

    /// spec `error_kinds_display`：数据结束、参数非法、未实现文本非空。
    #[test]
    fn error_kinds_display() {
        let data_ended = Error::DataEnded;
        let invalid_parameter = Error::invalid_parameter("odd hex length");
        let not_implemented = Error::NotImplemented;

        assert!(!data_ended.to_string().is_empty());
        assert!(!invalid_parameter.to_string().is_empty());
        assert!(!not_implemented.to_string().is_empty());
        assert_eq!(data_ended.partition(), ErrorPartition::DataOrRoute);
        assert_eq!(invalid_parameter.partition(), ErrorPartition::Parameter);
        assert_eq!(not_implemented.partition(), ErrorPartition::Logic);
    }

    /// spec `error_user_range_reserved`：自定义错误落在用户分区，不与库变体冲突。
    #[test]
    fn error_user_range_reserved() {
        let user = Error::user_defined(10_000, "application-specific");
        let library = Error::NotImplemented;

        assert_eq!(user.partition(), ErrorPartition::UserDefined);
        assert_ne!(user, library);
        assert!(!matches!(user, Error::NotImplemented));
        assert!(user.to_string().contains("10000"));
        assert!(user.to_string().contains("application-specific"));
    }

    #[test]
    fn remaining_variants_have_nonempty_display_and_partitions() {
        let cases = [
            (Error::NotAvailable, ErrorPartition::DataOrRoute),
            (Error::RouteNotFound, ErrorPartition::DataOrRoute),
            (Error::NotInitialized, ErrorPartition::Logic),
            (Error::system("permission denied"), ErrorPartition::System),
        ];
        for (error, partition) in cases {
            assert!(!error.to_string().is_empty());
            assert_eq!(error.partition(), partition);
        }
    }
}
