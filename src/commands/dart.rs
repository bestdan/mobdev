pub mod dcm;

use anyhow::Result;
use crate::utils::dart::{is_dart_package, find_dart_package_root, find_file_package_root, is_generated_dart_file};
use crate::utils::git::{is_git_repo, get_changed_files};

pub fn check(path: Option<String>, verbose: bool) -> Result<()> {
    let target_path = path.as_deref();
    let is_package = is_dart_package(target_path);

    if is_package {
        if verbose {
            if let Some(root) = find_dart_package_root(target_path) {
                eprintln!("✓ This is a Dart package");
                eprintln!("  Root: {}", root.display());
            }
        }
        Ok(())
    } else {
        if verbose {
            eprintln!("✗ Not a Dart package");
        }
        std::process::exit(1);
    }
}

pub fn root(path: Option<String>, verbose: bool) -> Result<()> {
    let target_path = path.as_deref();

    if let Some(root) = find_dart_package_root(target_path) {
        if verbose {
            eprintln!("Dart package root: {}", root.display());
        }
        println!("{}", root.display());
        Ok(())
    } else {
        if verbose {
            eprintln!("Error: Not in a Dart package");
        }
        std::process::exit(1);
    }
}

pub fn package(file: &str, verbose: bool) -> Result<()> {
    if let Some(root) = find_file_package_root(file) {
        if verbose {
            eprintln!("Package root for {}: {}", file, root.display());
        }
        println!("{}", root.display());
        Ok(())
    } else {
        if verbose {
            eprintln!("Error: Could not find Dart package for {}", file);
        }
        std::process::exit(1);
    }
}

pub fn changed(
    staged: bool,
    unstaged: bool,
    all: bool,
    base_branch: &str,
    verbose: bool,
) -> Result<()> {
    if !is_git_repo(None::<&str>) {
        if verbose {
            eprintln!("Error: Not in a git repository");
        }
        std::process::exit(1);
    }

    let files = get_changed_files(None::<&str>, base_branch, staged, unstaged, all)?;
    
    // Filter to only Dart files, excluding generated ones
    let dart_files: Vec<String> = files
        .into_iter()
        .filter(|f| f.ends_with(".dart") && !is_generated_dart_file(f))
        .collect();

    if verbose {
        eprintln!("Changed Dart files (vs {}):", base_branch);
        eprintln!("Count: {}", dart_files.len());
    }

    for file in dart_files {
        println!("{}", file);
    }

    Ok(())
}

pub fn changed_downstream(
    staged: bool,
    unstaged: bool,
    all: bool,
    base_branch: &str,
    relative: bool,
    verbose: bool,
) -> Result<()> {
    if verbose {
        eprintln!("Downstream analysis not yet fully implemented");
        eprintln!("Base branch: {}", base_branch);
        eprintln!("Relative: {}", relative);
    }

    // For now, just return the changed files
    changed(staged, unstaged, all, base_branch, false)
}

pub fn fix(
    verbose: bool,
    files: Option<Vec<String>>,
    apply: bool,
    packages: bool,
) -> Result<()> {
    if verbose {
        eprintln!("Dart fix functionality");
        if let Some(ref f) = files {
            eprintln!("Files: {:?}", f);
        }
        eprintln!("Apply: {}", apply);
        eprintln!("Packages: {}", packages);
    }

    anyhow::bail!("Dart fix not yet implemented");
}
