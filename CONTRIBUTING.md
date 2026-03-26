# Contributing

## Adding or changing commands

The CLI is defined in `src/main.rs` and `src/commands/`. After modifying any command, subcommand, or doc comment, regenerate the README:

```sh
cargo run -- generate-docs > README.md
```

The pre-commit hook enforces this — commits will be rejected if `README.md` is out of date.
