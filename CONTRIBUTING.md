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
4. Test: `cargo nextest run --all-features --no-tests warn`

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
- `cargo nextest run --all-features --no-tests warn` is green
- `cargo deny check` passes
- `typos` finds no issues

For changes headed to GitHub, run the full local gate before push so it matches CI as closely as practical.

Or simply run `pre-commit run --all-files`.

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0-only.
