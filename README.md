# nz

[简体中文](README.zh.md)

Rust revival of Laurent Constantin's **netwib + netwox + netwag 5.39.0**, covering the full capability set and later adding modern protocols.

Repository: <https://github.com/rcrwhyg/nz>
License: [GPL-3.0-only](LICENSE)

## Three layers

| Layer | Original | crate | Role |
|-------|----------|-------|------|
| Library | netwib | `nz-net` | Network capabilities, not a C API clone |
| CLI | netwox | `nz` | Numeric tool IDs plus named subcommands |
| GUI | netwag | `nz-gui` | Native **egui** workbench, no webview |

Current gate: agent skills (`.cursor/skills/`). Tools are not implemented yet.

Keep `README.md` (English, default) and `README.zh.md` in sync.

## Build

Requires Rust stable (see `rust-toolchain.toml`).

```bash
cargo build
pre-commit install
pre-commit run --all-files
```

The local `netw-ib-ox-ag-5.39.0/` tree is read-only and gitignored.

## Authorized use

Sniffing, spoofing, and scanning are only for this machine, a lab, or a written-authorized target.

## Docs

- [AGENTS.md](AGENTS.md) — scope, gates, constraints
- [CONTRIBUTING.md](CONTRIBUTING.md) — contributing and quality gates
- [SECURITY.md](SECURITY.md) — vulnerability reporting
- [doc/](doc/README.md) — extracted reference notes
- [spec/](spec/README.md) — task specs
- [skills/](skills/README.md) — agent skill index
