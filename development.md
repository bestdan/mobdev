## Development

### Building

```sh
cargo build
```

### Testing

```sh
cargo test
```

### Linting and Formatting

Check code formatting:
```sh
cargo fmt --check
```

Apply code formatting:
```sh
cargo fmt
```

Run linter (Clippy):
```sh
cargo clippy --all-targets --all-features -- -D warnings
```

Run all checks (format, lint, and test):
```sh
cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test
```

Or use the convenience script:
```sh
./check.sh
```

### Running

```sh
cargo run -- git check
```
