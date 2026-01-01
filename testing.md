## Testing

Run tests with:

```bash
cargo test
```

### Test Coverage

This project uses [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) for code coverage analysis.

Install tarpaulin:
```bash
cargo install cargo-tarpaulin
```

Run coverage tests:
```bash
cargo tarpaulin
```

The project has a minimum coverage threshold configured in `Cargo.toml`. Coverage reports will be generated in both HTML and stdout formats.

View the HTML coverage report:
```bash
cargo tarpaulin --out Html
# Open tarpaulin-report.html in your browser
```

### Build

Build release binary:
```bash
cargo build --release
```
