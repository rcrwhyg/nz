//! 简易命令行分词（对齐 netwox cmdline 常用引号路径）。

use crate::ParseError;

/// 将一行命令文本拆成 argv 风格 token。
///
/// 支持双引号 / 单引号；引号内 `\\` 与转义下一字符。
///
/// # Errors
///
/// 未闭合引号或尾随反斜杠时失败。
pub fn tokenize(input: &str) -> Result<Vec<String>, ParseError> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut in_double = false;
    let mut in_single = false;

    while let Some(ch) = chars.next() {
        if in_double {
            match ch {
                '"' => in_double = false,
                '\\' => push_escaped(&mut chars, &mut current)?,
                other => current.push(other),
            }
            continue;
        }
        if in_single {
            match ch {
                '\'' => in_single = false,
                '\\' => push_escaped(&mut chars, &mut current)?,
                other => current.push(other),
            }
            continue;
        }

        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            '"' => in_double = true,
            '\'' => in_single = true,
            '\\' => push_escaped(&mut chars, &mut current)?,
            other => current.push(other),
        }
    }

    if in_double || in_single {
        return Err(ParseError::ArgFileTokenize(String::from("unclosed quote")));
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    Ok(tokens)
}

fn push_escaped(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    current: &mut String,
) -> Result<(), ParseError> {
    let Some(next) = chars.next() else {
        return Err(ParseError::ArgFileTokenize(String::from(
            "trailing backslash",
        )));
    };
    current.push(next);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn splits_simple_and_quoted() {
        let tokens = tokenize(r#"prog -a 1 --name "hello world" 'x y'"#).expect("ok");
        assert_eq!(
            tokens,
            vec!["prog", "-a", "1", "--name", "hello world", "x y"]
        );
    }
}
