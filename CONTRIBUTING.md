# Contributing to nz

Thank you for your interest in contributing!

## Development Setup

1. Install Rust (stable, latest)
2. Install dev tools:
   ```bash
   cargo install cargo-nextest cargo-deny typos-cli git-cliff cargo-llvm-cov
   brew install pre-commit
   pre-commit install
   ```
3. Build: `cargo build`

`pre-commit install` writes `.git/hooks/pre-commit` (the `.git` directory is hidden in most editors). Confirm with:

```bash
ls -l .git/hooks/pre-commit
```

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

`pre-commit install` makes `git commit` run that gate. To run it without committing:

```bash
pre-commit run --all-files
```

GitHub Actions is the remote counterpart. Do not add a second local shell script that duplicates these hooks.

If Actions fails after a local commit already passed hooks, fix the cause and `git commit --amend` into that same commit. If it was already pushed, update the remote with `git push --force-with-lease` only—never `--force`. Do not amend after later feature commits exist.

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0-only.
