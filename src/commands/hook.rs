pub mod format;
pub mod analysis;
pub mod fix;
pub mod dcm;
pub mod graphql;

use anyhow::Result;

pub fn collate(
    staged: bool,
    unstaged: bool,
    all: bool,
    base_branch: &str,
    dart_format: bool,
    dart_analysis: bool,
    dcm_analyze: bool,
    graphql: bool,
    codeowners: bool,
    verbose: bool,
) -> Result<()> {
    if verbose {
        eprintln!("Running collated hook checks...");
    }

    let mut failures = Vec::new();

    // Run checks if flags are set or if no flags are set (run all by default)
    let run_all = !dart_format && !dart_analysis && !dcm_analyze && !graphql && !codeowners;

    if run_all || dart_format {
        if verbose {
            eprintln!("Running dart format check...");
        }
        if format::check(staged, unstaged, all, base_branch, false).is_err() {
            failures.push("dart format");
        }
    }

    if run_all || dart_analysis {
        if verbose {
            eprintln!("Running dart analysis check...");
        }
        if analysis::check(staged, unstaged, all, base_branch, false).is_err() {
            failures.push("dart analysis");
        }
    }

    if run_all || dcm_analyze {
        if verbose {
            eprintln!("Running DCM analyze check...");
        }
        if dcm::analyze_check(staged, unstaged, all, base_branch, false).is_err() {
            failures.push("DCM analyze");
        }
    }

    if run_all || graphql {
        if verbose {
            eprintln!("Running GraphQL check...");
        }
        if graphql::check(staged, unstaged, all, base_branch, false).is_err() {
            failures.push("GraphQL");
        }
    }

    if run_all || codeowners {
        if verbose {
            eprintln!("Running codeowners check...");
        }
        if crate::commands::git::codeowners::check(false).is_err() {
            failures.push("codeowners");
        }
    }

    if !failures.is_empty() {
        if verbose {
            eprintln!("Failed checks: {}", failures.join(", "));
        }
        std::process::exit(1);
    }

    if verbose {
        eprintln!("All checks passed ✓");
    }

    Ok(())
}
