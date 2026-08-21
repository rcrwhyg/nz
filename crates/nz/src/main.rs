//! `nz` CLI：数字工具号与具名子命令双模式分发。

use nz::{DispatchRequest, dispatch, format_catalog, invoke_tool};

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    match dispatch(&argv) {
        Ok(DispatchRequest::Catalog) => {
            println!("{}", format_catalog());
        }
        Ok(DispatchRequest::Run {
            entry,
            tool_arguments,
        }) => {
            let code = invoke_tool(entry, &tool_arguments);
            if code != 0 {
                std::process::exit(code);
            }
        }
        Err(error) => {
            eprintln!("nz: {error}");
            std::process::exit(1);
        }
    }
}
