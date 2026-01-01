# Git Commands

Git-related utility commands for repository management.

## Commands

### `mobdev git check`

Check if the current directory is in a git repository (exit code only).

```bash
mobdev git check [PATH] [--verbose]
```

**Arguments:**
- `PATH` - Optional path to check (defaults to current directory)

**Options:**
- `-v, --verbose` - Show human-readable status messages (output to stderr)

**Exit Codes:**
- `0` - Directory is in a git repository
- `1` - Directory is not in a git repository

**Examples:**
```bash
# Check current directory
mobdev git check

# Check specific directory
mobdev git check /path/to/project

# With verbose output
mobdev git check -v
```

### `mobdev git root`

Get the root directory of the git repository.

```bash
mobdev git root [PATH] [--verbose]
```

**Arguments:**
- `PATH` - Optional path to check (defaults to current directory)

**Options:**
- `-v, --verbose` - Show human-readable label (output to stderr)

**Output:**
- Prints the absolute path to the git root on stdout

**Examples:**
```bash
# Get git root
mobdev git root

# Use in scripts
cd "$(mobdev git root)"
```

### `mobdev git branch`

Get the current git branch name.

```bash
mobdev git branch [PATH] [--verbose]
```

**Arguments:**
- `PATH` - Optional path to check (defaults to current directory)

**Options:**
- `-v, --verbose` - Show human-readable label (output to stderr)

**Output:**
- Prints the current branch name on stdout

**Examples:**
```bash
# Get current branch
mobdev git branch

# Use in scripts
BRANCH=$(mobdev git branch)
```

### `mobdev git is-main`

Check if current branch is main (exit code only).

```bash
mobdev git is-main [PATH] [--branch <NAME>] [--verbose]
```

**Arguments:**
- `PATH` - Optional path to check (defaults to current directory)

**Options:**
- `-b, --branch <NAME>` - Main branch name to check against (default: "main")
- `-v, --verbose` - Show human-readable status messages (output to stderr)

**Exit Codes:**
- `0` - Current branch is the main branch
- `1` - Current branch is not the main branch

**Examples:**
```bash
# Check if on main branch
mobdev git is-main

# Check against custom main branch name
mobdev git is-main --branch master
```

### `mobdev git changed`

Show files that have changed compared to main branch.

```bash
mobdev git changed [OPTIONS]
```

**Options:**
- `-s, --staged` - Show staged changes only
- `-u, --unstaged` - Show unstaged changes only
- `-a, --all` - Show all changes (committed, staged, and unstaged)
- `-p, --push` - Show files in commits that would be pushed to upstream
- `-b, --base-branch <BRANCH>` - Base branch to compare against (default: "main")
- `-v, --verbose` - Show headers and counts (output to stderr)

**Output:**
- Prints one file path per line on stdout

**Examples:**
```bash
# Show all changed files vs main
mobdev git changed

# Show only staged files
mobdev git changed --staged

# Show files to be pushed
mobdev git changed --push

# Compare against different branch
mobdev git changed --base-branch develop
```

### `mobdev git commit-msg`

Generate a commit message from staged changes using Claude (requires Claude CLI).

**Note:** This feature is not yet fully implemented in the Rust version.

### `mobdev git pr-description`

Generate a GitHub PR description from branch changes using Claude (requires Claude CLI).

**Note:** This feature is not yet fully implemented in the Rust version.

### `mobdev git codeowners check`

Check if CODEOWNERS files are in sync (suitable for CI checks).

**Note:** This feature is not yet fully implemented in the Rust version.

## Pipe-Friendly Design

All git commands follow a pipe-friendly design:
- Clean output to stdout for piping
- Human-readable messages to stderr
- Appropriate exit codes

**Examples:**
```bash
# Get changed Dart files
mobdev git changed | grep '\.dart$'

# Count changed files
mobdev git changed | wc -l

# Filter and process files
mobdev git changed | mobdev files filter suffix .g.dart
```
