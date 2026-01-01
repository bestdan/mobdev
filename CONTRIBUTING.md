# Contributing to Mobdev

Thank you for your interest in contributing to mobdev! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

- Rust 1.70.0 or later
- Git
- A GitHub account

### Setting Up Your Development Environment

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/mobdev.git
   cd mobdev
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/bestdan/mobdev.git
   ```
4. Build the project:
   ```bash
   cargo build
   ```
5. Run tests:
   ```bash
   cargo test
   ```

## Development Workflow

### Before Starting Work

1. Create a new branch for your feature or fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Keep your branch up to date with upstream:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

### Making Changes

1. Write your code following the project's style guidelines
2. Add tests for new functionality
3. Ensure all tests pass:
   ```bash
   cargo test
   ```
4. Check for compilation warnings:
   ```bash
   cargo build --release
   ```
5. Format your code:
   ```bash
   cargo fmt
   ```
6. Run clippy for linting:
   ```bash
   cargo clippy -- -D warnings
   ```

### Committing Your Changes

1. Write clear, descriptive commit messages
2. Follow the conventional commits format:
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation changes
   - `test:` for adding tests
   - `refactor:` for code refactoring
   - `chore:` for maintenance tasks

Example:
```bash
git commit -m "feat: add git stash command support"
```

### Submitting a Pull Request

1. Push your changes to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
2. Create a pull request on GitHub
3. Describe your changes in detail
4. Link any related issues

## Code Style Guidelines

### Rust Code

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write doc comments for public APIs
- Prefer explicit over implicit

### Testing

- Write unit tests for utility functions
- Write integration tests for CLI commands
- Aim for high test coverage
- Test edge cases and error conditions

Example unit test:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(example_function("input"), "expected_output");
    }
}
```

Example integration test:
```rust
use assert_cmd::Command;

#[test]
fn test_cli_command() {
    let mut cmd = Command::cargo_bin("mobdev").unwrap();
    cmd.arg("git").arg("check");
    cmd.assert().success();
}
```

### Documentation

- Add doc comments to public functions
- Update README.md for user-facing changes
- Update command documentation in `docs/`
- Include examples in doc comments

Example:
```rust
/// Checks if the given directory is in a git repository.
///
/// # Arguments
///
/// * `cwd` - Optional path to check. Defaults to current directory.
///
/// # Returns
///
/// Returns `true` if inside a git repo, `false` otherwise.
///
/// # Examples
///
/// ```
/// use mobdev::utils::git::is_git_repo;
/// let is_repo = is_git_repo(Some("/path/to/repo"));
/// ```
pub fn is_git_repo<P: AsRef<Path>>(cwd: Option<P>) -> bool {
    // implementation
}
```

## Project Structure

Understanding the project structure will help you navigate the codebase:

```
src/
├── main.rs           # Entry point
├── cli.rs            # CLI definition using clap
├── commands/         # Command implementations
│   ├── mod.rs        # Module declarations
│   ├── check.rs      # System checks
│   ├── git.rs        # Git utilities
│   ├── dart.rs       # Dart utilities
│   ├── files.rs      # File utilities
│   ├── hook.rs       # Git hook utilities
│   └── upgrade.rs    # Upgrade functionality
└── utils/            # Utility functions
    ├── mod.rs        # Module declarations
    ├── git.rs        # Git helper functions
    ├── dart.rs       # Dart helper functions
    ├── files.rs      # File helper functions
    └── shell.rs      # Shell utility functions
```

## Adding New Commands

To add a new command:

1. Add the command to `src/cli.rs`
2. Implement the command in the appropriate module
3. Add tests for the command
4. Update documentation

Example:
```rust
// In src/cli.rs
#[derive(Subcommand)]
enum GitCommands {
    // ... existing commands
    
    /// New command description
    NewCommand {
        #[arg(short, long)]
        option: bool,
    },
}

// Handle in the match statement
GitCommands::NewCommand { option } => git::new_command(option),
```

## Security

- Never commit secrets or sensitive data
- Validate all user inputs
- Use proper error handling
- Follow secure coding practices
- Report security issues privately

## Getting Help

- Open an issue for bugs or feature requests
- Ask questions in discussions
- Check existing issues before creating new ones

## Code Review Process

All contributions go through code review:

1. Automated checks run on every PR
2. Maintainers review the code
3. Address feedback and update PR
4. Once approved, changes are merged

## License

By contributing to mobdev, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors are recognized in the project. Thank you for your contributions!

## Questions?

Feel free to open an issue if you have questions about contributing.
