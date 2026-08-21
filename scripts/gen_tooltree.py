#!/usr/bin/env python3
"""Generate crates/nz/src/registry/tooltree_data.rs from netwox 5.39.0 sources."""

from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOOLTREE_C = ROOT / "netw-ib-ox-ag-5.39.0/src/netwox-src/src/modules/tool/tooltree.c"
TOOLS_DIR = ROOT / "netw-ib-ox-ag-5.39.0/src/netwox-src/src/tools"
OUT = ROOT / "crates/nz/src/registry/tooltree_data.rs"


def slug(name: str) -> str:
    return name.lower().replace("_", "-")


def parse_categories() -> dict[str, dict]:
    text = TOOLTREE_C.read_text(errors="ignore")
    blocks = re.split(r"NETWOX_TTN_CUR\(NETWOX_TOOLTREENODETYPE_([A-Z0-9_]+)\)\s*;", text)
    nodes: dict[str, dict] = {}
    index = 1
    while index + 1 < len(blocks):
        name = blocks[index]
        body = blocks[index + 1]
        index += 2
        info_line = re.search(r'NETWOX_TTN_INFO(a?)\("([^"]*)"\)', body)
        if not info_line:
            continue
        nodes[name] = {
            "description": info_line.group(2),
            "can_add_tool": info_line.group(1) == "a",
            "children": re.findall(
                r"NETWOX_TTN_SN\(NETWOX_TOOLTREENODETYPE_([A-Z0-9_]+)\)", body
            ),
        }
    return nodes


def parse_tool_placements() -> dict[int, list[str]]:
    tool_nodes: dict[int, list[str]] = {}
    for path in sorted(TOOLS_DIR.glob("*.c")):
        match = re.match(r"(\d+)\.c$", path.name)
        if not match:
            continue
        tool_id = int(match.group(1))
        text = path.read_text(errors="ignore")
        nodes_match = re.search(r"_nodes\[\]\s*=\s*\{([^}]+)\}", text)
        if not nodes_match:
            continue
        names = [
            name
            for name in re.findall(
                r"NETWOX_TOOLTREENODETYPE_([A-Z0-9_]+)", nodes_match.group(1)
            )
            if name != "END"
        ]
        tool_nodes[tool_id] = names
    return tool_nodes


def main() -> None:
    nodes = parse_categories()
    tool_nodes = parse_tool_placements()
    order = ["MAIN"] + [key for key in nodes if key != "MAIN"]

    lines: list[str] = [
        "//! Generated from netwox `tooltree.c` + `tools/*/nodes` — do not hand-edit.",
        "//! Regenerate: `python3 scripts/gen_tooltree.py`.",
        "",
        "/// Category node in the Search tree.",
        "#[derive(Clone, Copy, Debug, Eq, PartialEq)]",
        "pub struct TreeCategory {",
        "    /// Stable id (slug).",
        "    pub id: &'static str,",
        "    /// Display label.",
        "    pub label: &'static str,",
        "    /// Whether tools may attach directly.",
        "    pub can_add_tool: bool,",
        "    /// Child category ids.",
        "    pub child_categories: &'static [&'static str],",
        "}",
        "",
        "/// All category nodes (including `main`).",
        "pub static TREE_CATEGORIES: &[TreeCategory] = &[",
    ]

    for name in order:
        info = nodes[name]
        kids = ", ".join(f'"{slug(child)}"' for child in info["children"])
        lines.extend(
            [
                "    TreeCategory {",
                f'        id: "{slug(name)}",',
                f"        label: {json.dumps(info['description'])},",
                f"        can_add_tool: {str(info['can_add_tool']).lower()},",
                f"        child_categories: &[{kids}],",
                "    },",
            ]
        )
    lines.append("];")
    lines.append("")
    lines.append("/// Tool id → leaf category ids (may be multiple).")
    lines.append("pub static TOOL_TREE_PLACEMENTS: &[(u32, &[&str])] = &[")
    for tool_id in sorted(tool_nodes):
        names = tool_nodes[tool_id]
        if not names:
            continue
        ids = ", ".join(f'"{slug(name)}"' for name in names)
        lines.append(f"    ({tool_id}, &[{ids}]),")
    lines.append("];")
    lines.append("")

    OUT.write_text("\n".join(lines))
    print(f"wrote {OUT} ({OUT.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
