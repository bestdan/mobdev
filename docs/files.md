# Files Commands

File filtering and manipulation utilities.

## Commands

### `mobdev files filter suffix`

Filter files by removing those matching suffix patterns.

```bash
mobdev files filter suffix <SUFFIXES...> [--verbose]
```

**Arguments:**
- `<SUFFIXES...>` - One or more suffix patterns to filter out (e.g., `.g.dart`, `.freezed.dart`)

**Options:**
- `-v, --verbose` - Show filter statistics (output to stderr)

**Input:**
- Reads file paths from stdin (one per line)

**Output:**
- Writes filtered file paths to stdout (one per line)
- If verbose: Prints statistics to stderr

**Examples:**
```bash
# Filter generated Dart files
mobdev git changed | mobdev files filter suffix .g.dart .freezed.dart

# Filter multiple patterns
echo "file1.dart
file2.g.dart
file3.freezed.dart
file4.dart" | mobdev files filter suffix .g.dart .freezed.dart
# Output:
# file1.dart
# file4.dart

# With verbose output
mobdev git changed | mobdev files filter suffix .g.dart -v
```

## Common Use Cases

### Filtering Code Generation Files

Dart projects often have generated files that should be excluded from certain operations:

```bash
# Get changed files excluding generated ones
mobdev git changed | mobdev files filter suffix .g.dart .freezed.dart .gr.dart .gql.dart

# Format only non-generated files
mobdev git changed | mobdev files filter suffix .g.dart .freezed.dart | xargs dart format
```

### CI/CD Pipelines

```yaml
# Example: Lint only non-generated files
- name: Lint Dart files
  run: |
    mobdev git changed | \
    mobdev files filter suffix .g.dart .freezed.dart | \
    xargs -I {} dart analyze {}
```

### Git Hooks

```bash
#!/bin/bash
# Pre-commit hook: Check only source files

# Get staged files, filter generated ones, then check formatting
mobdev git changed --staged | \
  mobdev files filter suffix .g.dart .freezed.dart | \
  xargs dart format --set-exit-if-changed
```

## Pipe-Friendly Design

This command is designed to work seamlessly in Unix pipelines:

```bash
# Chain multiple filters
mobdev git changed | \
  grep '\.dart$' | \
  mobdev files filter suffix .g.dart | \
  wc -l

# Process filtered files
mobdev git changed | \
  mobdev files filter suffix .g.dart .freezed.dart | \
  while read file; do
    echo "Processing: $file"
    # Do something with $file
  done
```
