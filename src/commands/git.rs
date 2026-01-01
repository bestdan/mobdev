pub mod codeowners;

use crate::utils::git::{
    get_changed_files, get_current_branch, get_files_to_push, get_git_root, is_git_repo,
    is_main_branch,
};
use anyhow::Result;

pub fn check(path: Option<String>, verbose: bool) -> Result<()> {
    let target_path = path.as_deref().unwrap_or(".");
    let is_repo = is_git_repo(Some(target_path));

    if is_repo {
        if verbose {
            let root = get_git_root(Some(target_path))?;
            eprintln!("✓ This is a git repository");
            eprintln!("  Root: {}", root.display());
        }
        Ok(())
    } else {
        if verbose {
            eprintln!("✗ Not a git repository");
        }
        std::process::exit(1);
    }
}

pub fn root(path: Option<String>, verbose: bool) -> Result<()> {
    let target_path = path.as_deref().unwrap_or(".");

    if !is_git_repo(Some(target_path)) {
        if verbose {
            eprintln!("Error: Not in a git repository");
        }
        std::process::exit(1);
    }

    let root = get_git_root(Some(target_path))?;

    if verbose {
        eprintln!("Git root: {}", root.display());
    }

    println!("{}", root.display());
    Ok(())
}

pub fn branch(path: Option<String>, verbose: bool) -> Result<()> {
    let target_path = path.as_deref().unwrap_or(".");

    if !is_git_repo(Some(target_path)) {
        if verbose {
            eprintln!("Error: Not in a git repository");
        }
        std::process::exit(1);
    }

    let branch = get_current_branch(Some(target_path))?;

    if verbose {
        eprintln!("Current branch: {}", branch);
    }

    println!("{}", branch);
    Ok(())
}

pub fn is_main(path: Option<String>, main_branch: &str, verbose: bool) -> Result<()> {
    let target_path = path.as_deref().unwrap_or(".");

    if !is_git_repo(Some(target_path)) {
        if verbose {
            eprintln!("Error: Not in a git repository");
        }
        std::process::exit(1);
    }

    let is_main = is_main_branch(Some(target_path), main_branch)?;

    if is_main {
        if verbose {
            eprintln!("✓ Current branch is {}", main_branch);
        }
        Ok(())
    } else {
        if verbose {
            let current = get_current_branch(Some(target_path))?;
            eprintln!("✗ Current branch is {} (not {})", current, main_branch);
        }
        std::process::exit(1);
    }
}

pub fn changed(
    staged: bool,
    unstaged: bool,
    all: bool,
    push: bool,
    base_branch: &str,
    verbose: bool,
) -> Result<()> {
    if !is_git_repo(None::<&str>) {
        if verbose {
            eprintln!("Error: Not in a git repository");
        }
        std::process::exit(1);
    }

    let files = if push {
        get_files_to_push(None::<&str>)?
    } else {
        get_changed_files(None::<&str>, base_branch, staged, unstaged, all)?
    };

    if verbose {
        if push {
            eprintln!("Files to push:");
        } else if staged {
            eprintln!("Staged files:");
        } else if unstaged {
            eprintln!("Unstaged files:");
        } else {
            eprintln!("Changed files (vs {}):", base_branch);
        }
        eprintln!("Count: {}", files.len());
    }

    for file in files {
        println!("{}", file);
    }

    Ok(())
}

pub fn commit_msg(commit: bool, verbose: bool) -> Result<()> {
    if verbose {
        eprintln!("Commit message generation requires Claude CLI");
        eprintln!("This feature is not yet fully implemented");
    }

    if commit && verbose {
        eprintln!("Auto-commit flag set");
    }

    anyhow::bail!("Claude integration not yet implemented");
}

pub fn pr_description(base_branch: &str, verbose: bool) -> Result<()> {
    if verbose {
        eprintln!("PR description generation requires Claude CLI");
        eprintln!("Base branch: {}", base_branch);
        eprintln!("This feature is not yet fully implemented");
    }

    anyhow::bail!("Claude integration not yet implemented");
}
