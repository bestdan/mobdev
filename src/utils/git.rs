use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Checks if the given directory is inside a git repository.
pub fn is_git_repo<P: AsRef<Path>>(cwd: Option<P>) -> bool {
    let path = cwd
        .as_ref()
        .map(|p| p.as_ref())
        .unwrap_or_else(|| Path::new("."));

    if !path.exists() {
        return false;
    }

    Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .current_dir(path)
        .output()
        .map(|output| {
            output.status.success()
                && String::from_utf8_lossy(&output.stdout).trim() == "true"
        })
        .unwrap_or(false)
}

/// Gets the root directory of the git repository.
pub fn get_git_root<P: AsRef<Path>>(cwd: Option<P>) -> Result<PathBuf> {
    let path = cwd
        .as_ref()
        .map(|p| p.as_ref())
        .unwrap_or_else(|| Path::new("."));

    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .current_dir(path)
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        anyhow::bail!("Not in a git repository");
    }

    let root = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git output")?
        .trim()
        .to_string();

    Ok(PathBuf::from(root))
}

/// Gets the current git branch name.
pub fn get_current_branch<P: AsRef<Path>>(cwd: Option<P>) -> Result<String> {
    let path = cwd
        .as_ref()
        .map(|p| p.as_ref())
        .unwrap_or_else(|| Path::new("."));

    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(path)
        .output()
        .context("Failed to execute git command")?;

    if !output.status.success() {
        anyhow::bail!("Failed to get current branch");
    }

    let branch = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git output")?
        .trim()
        .to_string();

    Ok(branch)
}

/// Checks if the current branch is the main branch.
pub fn is_main_branch<P: AsRef<Path>>(cwd: Option<P>, main_branch: &str) -> Result<bool> {
    let current = get_current_branch(cwd)?;
    Ok(current == main_branch)
}

/// Helper function to execute git diff and collect files
fn git_diff_files<P: AsRef<Path>>(path: P, args: &[&str]) -> Result<Vec<String>> {
    let mut cmd = Command::new("git");
    cmd.arg("diff").arg("--name-only");
    
    for arg in args {
        cmd.arg(arg);
    }
    
    let output = cmd.current_dir(path.as_ref())
        .output()
        .context("Failed to execute git diff")?;

    if output.status.success() {
        let file_list = String::from_utf8_lossy(&output.stdout);
        Ok(file_list.lines().map(|s| s.to_string()).collect())
    } else {
        Ok(Vec::new())
    }
}

/// Gets changed files in the repository.
pub fn get_changed_files<P: AsRef<Path>>(
    cwd: Option<P>,
    base_branch: &str,
    staged: bool,
    unstaged: bool,
    all: bool,
) -> Result<Vec<String>> {
    let path = cwd
        .as_ref()
        .map(|p| p.as_ref())
        .unwrap_or_else(|| Path::new("."));

    let mut files = Vec::new();

    if all || (!staged && !unstaged) {
        // Get all changed files vs base branch
        files.extend(git_diff_files(path, &[&format!("{}...HEAD", base_branch)])?);
        
        // Add staged files
        files.extend(git_diff_files(path, &["--cached"])?);
        
        // Add unstaged files
        files.extend(git_diff_files(path, &[])?);
    } else if staged {
        files.extend(git_diff_files(path, &["--cached"])?);
    } else if unstaged {
        files.extend(git_diff_files(path, &[])?);
    }

    // Remove duplicates and empty strings
    files.sort();
    files.dedup();
    files.retain(|s| !s.is_empty());

    Ok(files)
}

/// Gets files that would be pushed to upstream.
pub fn get_files_to_push<P: AsRef<Path>>(cwd: Option<P>) -> Result<Vec<String>> {
    let path = cwd
        .as_ref()
        .map(|p| p.as_ref())
        .unwrap_or_else(|| Path::new("."));

    // Get the upstream branch
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("@{u}")
        .current_dir(path)
        .output()
        .context("Failed to get upstream branch")?;

    if !output.status.success() {
        anyhow::bail!("No upstream branch set");
    }

    let upstream = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Get files in commits that would be pushed
    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(format!("{}...HEAD", upstream))
        .current_dir(path)
        .output()
        .context("Failed to execute git diff")?;

    if !output.status.success() {
        anyhow::bail!("Failed to get files to push");
    }

    let file_list = String::from_utf8_lossy(&output.stdout);
    let files: Vec<String> = file_list
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_repo() {
        // This test will work if run in a git repo
        // In CI, the repo should be cloned
        let result = is_git_repo(None::<&str>);
        // We can't assert true/false without knowing the test environment
        // Just ensure it doesn't panic
        let _ = result;
    }
}
