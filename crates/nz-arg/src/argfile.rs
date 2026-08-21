//! `--argfile`：读文件、跳过注释、分词。

use crate::ParseError;
use crate::cmdline;
use std::path::Path;

/// 读取 argfile，返回不含伪程序名的参数列表。
///
/// 规则对齐 netwox conffile 读路径：跳过空行与 `#` 开头行，行内容以空格拼接后再分词。
///
/// # Errors
///
/// IO 或分词失败。
pub fn load_argfile_tokens(path: &Path) -> Result<Vec<String>, ParseError> {
    let text = std::fs::read_to_string(path).map_err(|error| ParseError::ArgFileIo {
        path: path.display().to_string(),
        message: error.to_string(),
    })?;

    let mut joined = String::from("programnameignored");
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        joined.push(' ');
        joined.push_str(trimmed);
    }

    let mut tokens = cmdline::tokenize(&joined)?;
    if tokens.is_empty() {
        return Ok(Vec::new());
    }
    // 丢弃伪 argv[0]
    tokens.remove(0);
    Ok(tokens)
}
