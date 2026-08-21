//! argv 解析状态机。

use crate::ParseError;
use crate::argfile;
use crate::schema::{ArgSchema, ValueKind};
use crate::value::ParsedArgs;
use std::path::Path;

/// 解析模式。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseMode {
    /// 普通 CLI：处理 help / argfile；kbd 报错。
    Cli,
    /// 工具 0 formupdate：抑制 help / argfile / kbd 特殊分支。
    FormUpdate,
}

/// 解析成功结果。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseOutcome {
    /// 得到参数值。
    Parsed(ParsedArgs),
    /// 用户请求帮助（库不打印正文）。
    Help {
        /// 是否包含 Advanced。
        include_advanced: bool,
    },
}

/// 解析工具参数（`args` **不含**程序名）。
///
/// # Errors
///
/// 见 [`ParseError`]。
pub fn parse(
    schema: &ArgSchema,
    args: &[impl AsRef<str>],
    mode: ParseMode,
) -> Result<ParseOutcome, ParseError> {
    let tokens: Vec<String> = args.iter().map(|item| item.as_ref().to_owned()).collect();
    let mut parsed = ParsedArgs::new_from_schema(schema)?;
    if let Some(help) = parse_into(schema, &tokens, mode, &mut parsed)? {
        Ok(help)
    } else {
        if mode == ParseMode::Cli {
            parsed.check_required(schema)?;
        }
        Ok(ParseOutcome::Parsed(parsed))
    }
}

fn parse_into(
    schema: &ArgSchema,
    tokens: &[String],
    mode: ParseMode,
    parsed: &mut ParsedArgs,
) -> Result<Option<ParseOutcome>, ParseError> {
    let mut index = 0;
    while index < tokens.len() {
        let token = &tokens[index];
        if token == "--" {
            index += 1;
            collect_more(schema, parsed, &tokens[index..])?;
            return Ok(None);
        }
        if token.starts_with("--") {
            if let Some(outcome) = handle_long(schema, tokens, mode, parsed, &mut index)? {
                return Ok(Some(outcome));
            }
            index += 1;
            continue;
        }
        if token.starts_with('+') {
            handle_plus(schema, token, parsed)?;
            index += 1;
            continue;
        }
        if token.starts_with('-') {
            handle_short(schema, tokens, parsed, &mut index)?;
            index += 1;
            continue;
        }
        // 位置参数：先填 Required，否则进 MORE
        if let Some(key) = parsed.first_unset_required_key(schema) {
            parsed.set_raw(schema, key, token)?;
            index += 1;
            continue;
        }
        collect_more(schema, parsed, &tokens[index..])?;
        return Ok(None);
    }
    Ok(None)
}

fn collect_more(
    schema: &ArgSchema,
    parsed: &mut ParsedArgs,
    rest: &[String],
) -> Result<(), ParseError> {
    if rest.is_empty() {
        return Ok(());
    }
    if !schema.allow_more() {
        return Err(ParseError::UnexpectedPositional(rest[0].clone()));
    }
    for token in rest {
        parsed.push_more(token.clone());
    }
    Ok(())
}

fn handle_long(
    schema: &ArgSchema,
    tokens: &[String],
    mode: ParseMode,
    parsed: &mut ParsedArgs,
    index: &mut usize,
) -> Result<Option<ParseOutcome>, ParseError> {
    let token = &tokens[*index];
    if token == "--" {
        return Ok(None);
    }

    if mode == ParseMode::Cli {
        if token == "--help" || token == "--?" {
            return Ok(Some(ParseOutcome::Help {
                include_advanced: false,
            }));
        }
        if token == "--help2" || token == "--??" {
            return Ok(Some(ParseOutcome::Help {
                include_advanced: true,
            }));
        }
        if token == "--argfile" {
            *index += 1;
            let Some(path) = tokens.get(*index) else {
                return Err(ParseError::ArgFileMissingPath);
            };
            let file_tokens = argfile::load_argfile_tokens(Path::new(path))?;
            if let Some(outcome) = parse_into(schema, &file_tokens, mode, parsed)? {
                return Ok(Some(outcome));
            }
            return Ok(None);
        }
        if token == "--kbd" || token.starts_with("--kbd-") {
            return Err(ParseError::InteractiveNotSupported);
        }
    }

    if let Some(name) = token.strip_prefix("--no-") {
        let spec = schema.resolve_long_name(name)?;
        if spec.value_kind != ValueKind::Bool {
            return Err(ParseError::UnknownOption(token.clone()));
        }
        parsed.set_bool(spec.key, false)?;
        return Ok(None);
    }

    let name = token.trim_start_matches("--");
    if name.is_empty() {
        return Err(ParseError::BareSign(token.clone()));
    }
    let spec = schema.resolve_long_name(name)?;
    if spec.value_kind == ValueKind::Bool {
        parsed.set_bool(spec.key, true)?;
        return Ok(None);
    }
    *index += 1;
    let Some(value) = tokens.get(*index) else {
        return Err(ParseError::MissingValue(token.clone()));
    };
    parsed.set_raw(schema, spec.key, value)?;
    Ok(None)
}

fn handle_plus(schema: &ArgSchema, token: &str, parsed: &mut ParsedArgs) -> Result<(), ParseError> {
    if token == "+" {
        return Err(ParseError::BareSign(token.to_owned()));
    }
    let keys = &token[1..];
    if keys.is_empty() {
        return Err(ParseError::BareSign(token.to_owned()));
    }
    for key in keys.chars() {
        let spec = schema
            .find_by_key(key)
            .ok_or_else(|| ParseError::UnknownOption(format!("+{key}")))?;
        if spec.value_kind != ValueKind::Bool {
            return Err(ParseError::NonBooleanInCluster {
                key,
                cluster: token.to_owned(),
            });
        }
        parsed.set_bool(key, false)?;
    }
    Ok(())
}

fn handle_short(
    schema: &ArgSchema,
    tokens: &[String],
    parsed: &mut ParsedArgs,
    index: &mut usize,
) -> Result<(), ParseError> {
    let token = &tokens[*index];
    if token == "-" {
        return Err(ParseError::BareSign(token.clone()));
    }
    let body = &token[1..];
    if body.is_empty() {
        return Err(ParseError::BareSign(token.clone()));
    }

    // 多字符：全部必须是布尔连写
    if body.chars().count() > 1 {
        for key in body.chars() {
            let spec = schema
                .find_by_key(key)
                .ok_or_else(|| ParseError::UnknownOption(format!("-{key}")))?;
            if spec.value_kind != ValueKind::Bool {
                return Err(ParseError::NonBooleanInCluster {
                    key,
                    cluster: token.clone(),
                });
            }
            parsed.set_bool(key, true)?;
        }
        return Ok(());
    }

    let key = body.chars().next().expect("single char");
    let spec = schema
        .find_by_key(key)
        .ok_or_else(|| ParseError::UnknownOption(format!("-{key}")))?;
    if spec.value_kind == ValueKind::Bool {
        parsed.set_bool(key, true)?;
        return Ok(());
    }
    *index += 1;
    let Some(value) = tokens.get(*index) else {
        return Err(ParseError::MissingValue(format!(
            "-{key}|--{}",
            spec.long_name
        )));
    };
    parsed.set_raw(schema, key, value)?;
    Ok(())
}
