# Check Commands

System dependency checks and environment validation.

## Commands

### `mobdev check externals`

Check if external dependencies (dart, dcm, melos, claude) are installed.

```bash
mobdev check externals [--verbose]
```

**Options:**
- `-v, --verbose` - Show human-readable status messages (output to stderr)

**Output:**
- If verbose: Prints status of each dependency to stderr
- Prints names of missing dependencies to stdout (one per line)

**Exit Codes:**
- `0` - All dependencies are installed
- `1` - One or more dependencies are missing

**Examples:**
```bash
# Check all external dependencies
mobdev check externals

# Check with verbose output
mobdev check externals -v

# Use in scripts
if mobdev check externals; then
    echo "All dependencies installed"
else
    echo "Missing dependencies:"
    mobdev check externals
fi
```

**Dependencies Checked:**
- `dart` - Dart SDK
- `dcm` - Dart Code Metrics
- `melos` - Melos monorepo management tool
- `claude` - Claude CLI for AI assistance

### `mobdev check version`

Check if mobdev is on the most recent version.

**Note:** This feature is not yet fully implemented in the Rust version.

```bash
mobdev check version [--verbose]
```

**Options:**
- `-v, --verbose` - Show human-readable status messages (output to stderr)

## Usage in CI/CD

These commands are designed to work well in CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Check dependencies
  run: |
    if ! mobdev check externals; then
      echo "Missing dependencies"
      exit 1
    fi
```

```bash
# Example pre-commit hook
#!/bin/bash
mobdev check externals || {
    echo "Error: Required dependencies not installed"
    exit 1
}
```
