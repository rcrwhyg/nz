//! 解析错误类型。

use thiserror::Error;

/// argv / schema 解析失败原因。
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseError {
    /// Schema 构建非法。
    #[error("{0}")]
    InvalidSchema(String),
    /// 未知短选项或长选项。
    #[error("unknown option: {0}")]
    UnknownOption(String),
    /// 长名前缀命中多个参数。
    #[error("ambiguous long option prefix '{prefix}' matches --{first} and --{second}")]
    AmbiguousPrefix {
        /// 用户输入前缀（不含 `--`）。
        prefix: String,
        /// 冲突长名之一。
        first: String,
        /// 冲突长名之二。
        second: String,
    },
    /// 取值选项缺少后续值。
    #[error("option '{0}' requires a value")]
    MissingValue(String),
    /// 值无法按类型解析。
    #[error("option '{option}' could not parse value '{value}'")]
    InvalidValue {
        /// 选项展示名。
        option: String,
        /// 原始值。
        value: String,
    },
    /// 必选参数未给出。
    #[error("required option '-{key}|--{long_name}' is missing")]
    MissingRequired {
        /// 短键。
        key: char,
        /// 长名。
        long_name: String,
    },
    /// 不允许 MORE 时仍有尾部参数。
    #[error("unexpected positional argument: {0}")]
    UnexpectedPositional(String),
    /// `--argfile` 缺少路径。
    #[error("option '--argfile' must be followed by a filename")]
    ArgFileMissingPath,
    /// 读取 argfile 失败。
    #[error("failed to read argfile '{path}': {message}")]
    ArgFileIo {
        /// 文件路径。
        path: String,
        /// 底层信息。
        message: String,
    },
    /// argfile 内命令行分词失败。
    #[error("failed to tokenize argfile content: {0}")]
    ArgFileTokenize(String),
    /// `--kbd` 交互未实现。
    #[error("interactive --kbd is not supported in nz-arg MVP")]
    InteractiveNotSupported,
    /// 裸 `-` 或 `+`。
    #[error("option '{0}' is not recognized")]
    BareSign(String),
    /// 短选项连写中含非布尔。
    #[error("option '{key}' is not boolean (in '{cluster}')")]
    NonBooleanInCluster {
        /// 出问题的短键。
        key: char,
        /// 整个 cluster token。
        cluster: String,
    },
}
