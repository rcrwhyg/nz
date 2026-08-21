//! `nz` CLI 库面：登记表与分发（二进制入口见 `main.rs`）。

pub mod registry;
pub mod tool_schemas;

pub use registry::{
    DEFERRED_TOOL_IDS, DispatchError, DispatchRequest, PublishKind, TOOL_ENTRIES, ToolEntry,
    ToolId, backspace_tool_ids, dispatch, format_catalog, invoke_stub, lookup_by_id,
    lookup_by_name, stdin_tool_ids, tools_for_search,
};
pub use tool_schemas::tool0_schema;
