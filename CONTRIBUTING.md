# Contributing

## Prerequisites

- Rust (stable)
- [husky](https://typicode.github.io/husky) (installed via `npm install` or `npx husky install`)

## Development workflow

```sh
cargo build       # build
cargo test        # run tests
cargo clippy      # lint
cargo fmt         # format
```

## Adding or changing commands

The CLI is defined in `src/main.rs` and `src/commands/`. After modifying any command, subcommand, or doc comment, regenerate the README:

```sh
cargo run -- generate-docs > README.md
```

The pre-commit hook enforces this — commits will be rejected if `README.md` is out of date.

## Hooks

| Hook         | Checks |
|--------------|--------|
| `pre-commit` | `cargo fmt --check`, `cargo clippy`, README up-to-date |
| `pre-push`   | `cargo fmt --check`, `cargo clippy`, `cargo test`, `cargo build --release` |
