//! 命令文件：读入、跳过注释、分词；读成功后删除。

use std::path::{Path, PathBuf};

/// 读命令文件得到 token 列表。
///
/// 跳过空行与 `#` 注释，行内容空格拼接后再分词。
/// 成功读到文件后**删除**该文件（即使随后解析失败）。读失败不删。
///
/// # Errors
///
/// IO 或分词失败。
pub fn read_command_file_then_delete(path: &Path) -> Result<Vec<String>, CommandFileError> {
    let text = std::fs::read_to_string(path).map_err(|error| CommandFileError::Io {
        path: path.to_path_buf(),
        message: error.to_string(),
    })?;

    std::fs::remove_file(path).map_err(|error| CommandFileError::Io {
        path: path.to_path_buf(),
        message: format!("read ok but delete failed: {error}"),
    })?;

    let mut joined = String::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if !joined.is_empty() {
            joined.push(' ');
        }
        joined.push_str(trimmed);
    }

    nz_arg::tokenize(&joined).map_err(CommandFileError::Tokenize)
}

/// 命令文件错误。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandFileError {
    /// 读写失败。
    Io {
        /// 路径。
        path: PathBuf,
        /// 说明。
        message: String,
    },
    /// 分词失败。
    Tokenize(nz_arg::ParseError),
}

impl std::fmt::Display for CommandFileError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, message } => {
                write!(formatter, "command file '{}': {message}", path.display())
            }
            Self::Tokenize(error) => write!(formatter, "command file tokenize: {error}"),
        }
    }
}

impl std::error::Error for CommandFileError {}
