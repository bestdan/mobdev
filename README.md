# mobdev

Mobile developer utility package - Rust implementation of TSU (TypeScript Utilities).

## Installation

### From Source

```sh
git clone https://github.com/bestdan/mobdev.git
cd mobdev
cargo build --release
cargo install --path .
```

Or install directly from GitHub:

```sh
cargo install --git https://github.com/bestdan/mobdev
```

### Pre-built Binaries

Pre-built binaries will be available in the releases page.

## Usage

```sh
mobdev <namespace> <command> [options]
```

## Available Commands

### Available Namespaces

- **check** - System dependency checks
- **upgrade** - Check for newer versions
- **git** - Git-related utilities
- **dart** - Dart/Flutter project utilities
- **hook** - Git / Claude hook utilities for Dart
- **files** - File filtering utilities

### Example Commands

```bash
# Check if in a git repository
mobdev git check

# Get git root path
mobdev git root

# Show changed files
mobdev git changed

# Check external dependencies
mobdev check externals --verbose

# Filter files by extension
mobdev git changed | mobdev files filter suffix .g.dart .freezed.dart
```

### Command Design Philosophy

All commands follow a **pipe-friendly** design:

- Clean, parseable output to **stdout**
- Error messages to **stderr**
- Appropriate exit codes
- `--verbose` flag for debugging (outputs to stderr)

## Requirements
- **Rust**: >=1.70.0 (for building from source)

### Optional Dependencies

Some commands require additional tools:

- **Claude CLI**: For `git commit-msg` and `git pr-description` - [Install](https://github.com/anthropics/claude-cli)
- **Dart SDK**: For `dart` commands - [Install](https://dart.dev)
- **DCM**: For `hook dcm check` - [Install](https://dcm.dev)
- **Melos**: For `hook graphql check` - [Install](https://melos.invertase.dev)

`
## License

MIT

