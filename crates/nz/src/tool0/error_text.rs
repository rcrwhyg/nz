//! 工具 0 `--error`：把数值码格式化为可读文本（不与 C 1:1）。

/// 对照 netwib 分区语义的粗粒度说明。
#[must_use]
pub fn describe(code: u32) -> String {
    if code == 0 {
        return String::from("ok");
    }
    if (1000..2000).contains(&code) {
        return format!("data or route error ({code})");
    }
    if (2000..3000).contains(&code) {
        return format!("invalid parameter ({code})");
    }
    if (3000..4000).contains(&code) {
        return format!("logic error ({code})");
    }
    if (4000..10000).contains(&code) {
        return format!("system error ({code})");
    }
    if code >= 10_000 {
        return format!("user-defined error ({code})");
    }
    format!("unknown error code ({code})")
}
