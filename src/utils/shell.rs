use anyhow::{Result, bail};
use regex::Regex;
use std::process::Command;

/// Regex pattern for validating safe shell inputs.
/// Allows: alphanumeric, dots, underscores, slashes, spaces, and dashes.
/// Rejects: quotes, backticks, dollar signs, semicolons, pipes, ampersands, redirects, wildcards, etc.
const SAFE_SHELL_INPUT_PATTERN: &str = r"^[a-zA-Z0-9._/\s-]+$";

/// Escapes a shell argument to prevent injection attacks.
/// Uses single-quote escaping which is safe for most shells.
pub fn escape_shell_arg(arg: &str) -> String {
    format!("'{}'", arg.replace('\'', r"'\''"))
}

/// Validates that a string contains only safe characters for use in shell commands.
pub fn is_safe_shell_input(input: &str) -> bool {
    let re = Regex::new(SAFE_SHELL_INPUT_PATTERN).unwrap();
    re.is_match(input)
}

/// Validates and escapes a shell argument.
/// Throws an error if the input contains potentially dangerous characters.
pub fn safe_shell_arg(arg: &str, allow_unsafe: bool) -> Result<String> {
    if !allow_unsafe && !is_safe_shell_input(arg) {
        bail!(
            "Unsafe shell argument detected: \"{}\". Contains potentially dangerous characters.",
            arg
        );
    }
    Ok(escape_shell_arg(arg))
}

/// Checks if a command is available in the system PATH.
pub fn is_command_installed(command: &str) -> bool {
    Command::new("command")
        .arg("-v")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_shell_input() {
        assert!(is_safe_shell_input("myfile.txt"));
        assert!(is_safe_shell_input("path/to/file"));
        assert!(!is_safe_shell_input("file; rm -rf /"));
        assert!(!is_safe_shell_input("file$variable"));
        assert!(!is_safe_shell_input("file`command`"));
    }

    #[test]
    fn test_escape_shell_arg() {
        assert_eq!(escape_shell_arg("simple"), "'simple'");
        assert_eq!(escape_shell_arg("file's name"), "'file'\\''s name'");
    }

    #[test]
    fn test_safe_shell_arg() {
        assert!(safe_shell_arg("safe.txt", false).is_ok());
        assert!(safe_shell_arg("unsafe; rm", false).is_err());
        assert!(safe_shell_arg("unsafe; rm", true).is_ok());
    }
}
