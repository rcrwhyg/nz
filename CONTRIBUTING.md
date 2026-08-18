# Contributing to nz

Thank you for your interest in contributing!

## Development Setup

1. Install Rust (stable, latest)
2. Install dev tools:
   ```bash
   cargo install cargo-nextest cargo-deny typos-cli git-cliff cargo-llvm-cov
   pip install pre-commit
   pre-commit install
   ```
3. Build: `cargo build`
4. Test: `cargo nextest run --all-features`

## Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat(scope):` new feature
- `fix(scope):` bug fix
- `docs(scope):` documentation
- `refactor(scope):` code restructuring
- `test(scope):` test additions/changes
- `ci(scope):` CI/CD changes

## Quality Gates

Before submitting a PR, ensure:

- `cargo fmt --all -- --check` passes
- `cargo clippy --all-targets --all-features -- -D warnings` is clean
- `cargo nextest run --all-features` all green
- `cargo deny check` passes
- `typos` finds no issues

Or simply run `pre-commit run --all-files`.

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0-only.
