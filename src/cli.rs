use clap::{Parser, Subcommand};
use anyhow::Result;
use std::process;

use crate::commands::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "mobdev")]
#[command(bin_name = "mobdev")]
#[command(about = "Mobile developer utility package - Rust implementation of TSU", long_about = None)]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check system dependencies and environment
    Check {
        #[command(subcommand)]
        command: CheckCommands,
    },
    /// Upgrade mobdev to the latest version from GitHub
    Upgrade {
        /// Show progress messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
        /// Package manager to use (cargo, or other)
        #[arg(short, long, default_value = "cargo")]
        package_manager: String,
    },
    /// Git repository utilities
    Git {
        #[command(subcommand)]
        command: GitCommands,
    },
    /// File manipulation utilities
    Files {
        #[command(subcommand)]
        command: FilesCommands,
    },
    /// Dart package utilities
    Dart {
        #[command(subcommand)]
        command: DartCommands,
    },
    /// Git hook utilities for Dart
    Hook {
        #[command(subcommand)]
        command: HookCommands,
    },
}

#[derive(Subcommand)]
enum CheckCommands {
    /// Check if external dependencies (dart, dcm, melos, claude) are installed
    Externals {
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Check if mobdev is on the most recent version
    Version {
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum GitCommands {
    /// Check if current directory is in a git repository (exit code only)
    Check {
        /// Path to check (defaults to current directory)
        path: Option<String>,
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Get the root directory of the git repository
    Root {
        /// Path to check (defaults to current directory)
        path: Option<String>,
        /// Show human-readable label (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Get the current git branch name
    Branch {
        /// Path to check (defaults to current directory)
        path: Option<String>,
        /// Show human-readable label (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Check if current branch is main (exit code only)
    IsMain {
        /// Path to check (defaults to current directory)
        path: Option<String>,
        /// Main branch name to check against
        #[arg(short, long, default_value = "main")]
        branch: String,
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Show files that have changed compared to main branch
    Changed {
        /// Show staged changes only
        #[arg(short, long)]
        staged: bool,
        /// Show unstaged changes only
        #[arg(short, long)]
        unstaged: bool,
        /// Show all changes (committed, staged, and unstaged)
        #[arg(short, long)]
        all: bool,
        /// Show files in commits that would be pushed to upstream
        #[arg(short, long)]
        push: bool,
        /// Base branch to compare against
        #[arg(short, long, default_value = "main")]
        base_branch: String,
        /// Show headers and counts (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Generate a commit message from staged changes using Claude
    CommitMsg {
        /// Automatically create the commit with generated message
        #[arg(short, long)]
        commit: bool,
        /// Show progress messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Generate a GitHub PR description from branch changes using Claude
    PrDescription {
        /// Base branch to compare against
        #[arg(short, long, default_value = "main")]
        base_branch: String,
        /// Show progress messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
    /// CODEOWNERS file utilities
    Codeowners {
        #[command(subcommand)]
        command: CodeownersCommands,
    },
}

#[derive(Subcommand)]
enum CodeownersCommands {
    /// Check if CODEOWNERS files are in sync (suitable for CI checks)
    Check {
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum FilesCommands {
    /// Filter files from stdin
    Filter {
        #[command(subcommand)]
        command: FilterCommands,
    },
}

#[derive(Subcommand)]
enum FilterCommands {
    /// Filter files by removing those matching suffix patterns
    Suffix {
        /// Suffix patterns to filter out (e.g., .g.dart .gql.dart)
        suffixes: Vec<String>,
        /// Show filter statistics (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum DartCommands {
    /// Check if current directory is in a Dart package (exit code only)
    Check {
        /// Path to check (defaults to current directory)
        path: Option<String>,
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Get the root directory of the Dart package
    Root {
        /// Path to check (defaults to current directory)
        path: Option<String>,
        /// Show human-readable label (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Get the package root containing a specific file (useful in mono-repos)
    Package {
        /// Path to the file
        file: String,
        /// Show human-readable label (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
    /// Show Dart files that have changed
    Changed {
        /// Show staged changes only
        #[arg(short, long)]
        staged: bool,
        /// Show unstaged changes only
        #[arg(short, long)]
        unstaged: bool,
        /// Show all changes (committed, staged, and unstaged)
        #[arg(short, long)]
        all: bool,
        /// Base branch to compare against
        #[arg(short, long, default_value = "main")]
        base_branch: String,
        /// Show headers and counts (output to stderr)
        #[arg(short, long)]
        verbose: bool,
        #[command(subcommand)]
        command: Option<DartChangedCommands>,
    },
    /// Run dart fix (dry-run by default)
    Fix {
        /// Show detailed progress information
        #[arg(short, long)]
        verbose: bool,
        /// Specific files or directories to check
        #[arg(short, long, num_args = 1..)]
        files: Option<Vec<String>>,
        /// Apply fixes automatically (default is dry-run)
        #[arg(long)]
        apply: bool,
        /// Run on affected packages instead of individual files
        #[arg(long)]
        packages: bool,
    },
    /// DCM code quality utilities
    Dcm {
        #[command(subcommand)]
        command: DcmCommands,
    },
}

#[derive(Subcommand)]
enum DartChangedCommands {
    /// Find all Dart files that depend on changed Dart files
    Downstream {
        /// Analyze staged changes only
        #[arg(short, long)]
        staged: bool,
        /// Analyze unstaged changes only
        #[arg(short, long)]
        unstaged: bool,
        /// Analyze all changes (committed, staged, and unstaged)
        #[arg(short, long)]
        all: bool,
        /// Base branch to compare against
        #[arg(short, long, default_value = "main")]
        base_branch: String,
        /// Output relative paths instead of absolute paths
        #[arg(long)]
        relative: bool,
        /// Show detailed progress information (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum DcmCommands {
    /// Run DCM analyze and output files with issues
    Analyze {
        /// Show detailed progress information
        #[arg(short, long)]
        verbose: bool,
        /// Timeout in milliseconds
        #[arg(long, default_value = "7000")]
        timeout: u64,
    },
}

#[derive(Subcommand)]
enum HookCommands {
    /// Check if Dart files are properly formatted (suitable for pre-push hooks)
    Format {
        #[command(subcommand)]
        command: FormatCommands,
    },
    /// Check if Dart files pass dart analyze (suitable for pre-push hooks)
    Analysis {
        #[command(subcommand)]
        command: AnalysisCommands,
    },
    /// Check if Dart files pass dart fix and apply fixes (suitable for pre-push hooks)
    Fix {
        #[command(subcommand)]
        command: FixCommands,
    },
    /// DCM utilities for Dart code quality
    Dcm {
        #[command(subcommand)]
        command: HookDcmCommands,
    },
    /// Check if GraphQL fakes are up to date (suitable for pre-push hooks)
    Graphql {
        #[command(subcommand)]
        command: GraphqlCommands,
    },
    /// Run multiple hook checks and track failures (suitable for pre-push hooks)
    Collate {
        /// Check staged changes only
        #[arg(short, long)]
        staged: bool,
        /// Check unstaged changes only
        #[arg(short, long)]
        unstaged: bool,
        /// Check all changes (committed, staged, and unstaged)
        #[arg(short, long)]
        all: bool,
        /// Base branch to compare against
        #[arg(short, long, default_value = "main")]
        base_branch: String,
        /// Run dart format check
        #[arg(long)]
        dart_format: bool,
        /// Run dart analysis check
        #[arg(long)]
        dart_analysis: bool,
        /// Run DCM analyze check
        #[arg(long)]
        dcm_analyze: bool,
        /// Run GraphQL check
        #[arg(long)]
        graphql: bool,
        /// Run git codeowners check
        #[arg(long)]
        codeowners: bool,
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum FormatCommands {
    Check {
        /// Check staged changes only
        #[arg(short, long)]
        staged: bool,
        /// Check unstaged changes only
        #[arg(short, long)]
        unstaged: bool,
        /// Check all changes (committed, staged, and unstaged)
        #[arg(short, long)]
        all: bool,
        /// Base branch to compare against
        #[arg(short, long, default_value = "main")]
        base_branch: String,
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum AnalysisCommands {
    Check {
        /// Check staged changes only
        #[arg(short, long)]
        staged: bool,
        /// Check unstaged changes only
        #[arg(short, long)]
        unstaged: bool,
        /// Check all changes (committed, staged, and unstaged)
        #[arg(short, long)]
        all: bool,
        /// Base branch to compare against
        #[arg(short, long, default_value = "main")]
        base_branch: String,
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum FixCommands {
    Check {
        /// Check staged changes only
        #[arg(short, long)]
        staged: bool,
        /// Check unstaged changes only
        #[arg(short, long)]
        unstaged: bool,
        /// Check all changes (committed, staged, and unstaged)
        #[arg(short, long)]
        all: bool,
        /// Base branch to compare against
        #[arg(short, long, default_value = "main")]
        base_branch: String,
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum HookDcmCommands {
    /// Check if Dart files pass DCM fix checks (suitable for pre-push hooks)
    Fix {
        #[command(subcommand)]
        command: DcmFixCommands,
    },
    /// Check if Dart files pass DCM analyze checks (suitable for pre-push hooks)
    Analyze {
        #[command(subcommand)]
        command: DcmAnalyzeCommands,
    },
}

#[derive(Subcommand)]
enum DcmFixCommands {
    Check {
        /// Check staged changes only
        #[arg(short, long)]
        staged: bool,
        /// Check unstaged changes only
        #[arg(short, long)]
        unstaged: bool,
        /// Check all changes (committed, staged, and unstaged)
        #[arg(short, long)]
        all: bool,
        /// Base branch to compare against
        #[arg(short, long, default_value = "main")]
        base_branch: String,
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum DcmAnalyzeCommands {
    Check {
        /// Check staged changes only
        #[arg(short, long)]
        staged: bool,
        /// Check unstaged changes only
        #[arg(short, long)]
        unstaged: bool,
        /// Check all changes (committed, staged, and unstaged)
        #[arg(short, long)]
        all: bool,
        /// Base branch to compare against
        #[arg(short, long, default_value = "main")]
        base_branch: String,
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Subcommand)]
enum GraphqlCommands {
    Check {
        /// Check staged changes only
        #[arg(short, long)]
        staged: bool,
        /// Check unstaged changes only
        #[arg(short, long)]
        unstaged: bool,
        /// Check all changes (committed, staged, and unstaged)
        #[arg(short, long)]
        all: bool,
        /// Base branch to compare against
        #[arg(short, long, default_value = "main")]
        base_branch: String,
        /// Show human-readable status messages (output to stderr)
        #[arg(short, long)]
        verbose: bool,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Check { command } => match command {
            CheckCommands::Externals { verbose } => check::externals(verbose),
            CheckCommands::Version { verbose } => check::version(verbose),
        },
        Commands::Upgrade {
            verbose,
            package_manager,
        } => upgrade::upgrade(verbose, &package_manager),
        Commands::Git { command } => match command {
            GitCommands::Check { path, verbose } => git::check(path, verbose),
            GitCommands::Root { path, verbose } => git::root(path, verbose),
            GitCommands::Branch { path, verbose } => git::branch(path, verbose),
            GitCommands::IsMain {
                path,
                branch,
                verbose,
            } => git::is_main(path, &branch, verbose),
            GitCommands::Changed {
                staged,
                unstaged,
                all,
                push,
                base_branch,
                verbose,
            } => git::changed(staged, unstaged, all, push, &base_branch, verbose),
            GitCommands::CommitMsg { commit, verbose } => git::commit_msg(commit, verbose),
            GitCommands::PrDescription {
                base_branch,
                verbose,
            } => git::pr_description(&base_branch, verbose),
            GitCommands::Codeowners { command } => match command {
                CodeownersCommands::Check { verbose } => git::codeowners::check(verbose),
            },
        },
        Commands::Files { command } => match command {
            FilesCommands::Filter { command } => match command {
                FilterCommands::Suffix { suffixes, verbose } => {
                    files::filter::suffix(suffixes, verbose)
                }
            },
        },
        Commands::Dart { command } => match command {
            DartCommands::Check { path, verbose } => dart::check(path, verbose),
            DartCommands::Root { path, verbose } => dart::root(path, verbose),
            DartCommands::Package { file, verbose } => dart::package(&file, verbose),
            DartCommands::Changed {
                staged,
                unstaged,
                all,
                base_branch,
                verbose,
                command,
            } => {
                if let Some(DartChangedCommands::Downstream {
                    staged,
                    unstaged,
                    all,
                    base_branch,
                    relative,
                    verbose,
                }) = command
                {
                    dart::changed_downstream(staged, unstaged, all, &base_branch, relative, verbose)
                } else {
                    dart::changed(staged, unstaged, all, &base_branch, verbose)
                }
            }
            DartCommands::Fix {
                verbose,
                files,
                apply,
                packages,
            } => dart::fix(verbose, files, apply, packages),
            DartCommands::Dcm { command } => match command {
                DcmCommands::Analyze { verbose, timeout } => dart::dcm::analyze(verbose, timeout),
            },
        },
        Commands::Hook { command } => match command {
            HookCommands::Format { command } => match command {
                FormatCommands::Check {
                    staged,
                    unstaged,
                    all,
                    base_branch,
                    verbose,
                } => hook::format::check(staged, unstaged, all, &base_branch, verbose),
            },
            HookCommands::Analysis { command } => match command {
                AnalysisCommands::Check {
                    staged,
                    unstaged,
                    all,
                    base_branch,
                    verbose,
                } => hook::analysis::check(staged, unstaged, all, &base_branch, verbose),
            },
            HookCommands::Fix { command } => match command {
                FixCommands::Check {
                    staged,
                    unstaged,
                    all,
                    base_branch,
                    verbose,
                } => hook::fix::check(staged, unstaged, all, &base_branch, verbose),
            },
            HookCommands::Dcm { command } => match command {
                HookDcmCommands::Fix { command } => match command {
                    DcmFixCommands::Check {
                        staged,
                        unstaged,
                        all,
                        base_branch,
                        verbose,
                    } => hook::dcm::fix_check(staged, unstaged, all, &base_branch, verbose),
                },
                HookDcmCommands::Analyze { command } => match command {
                    DcmAnalyzeCommands::Check {
                        staged,
                        unstaged,
                        all,
                        base_branch,
                        verbose,
                    } => hook::dcm::analyze_check(staged, unstaged, all, &base_branch, verbose),
                },
            },
            HookCommands::Graphql { command } => match command {
                GraphqlCommands::Check {
                    staged,
                    unstaged,
                    all,
                    base_branch,
                    verbose,
                } => hook::graphql::check(staged, unstaged, all, &base_branch, verbose),
            },
            HookCommands::Collate {
                staged,
                unstaged,
                all,
                base_branch,
                dart_format,
                dart_analysis,
                dcm_analyze,
                graphql,
                codeowners,
                verbose,
            } => hook::collate(
                staged,
                unstaged,
                all,
                &base_branch,
                dart_format,
                dart_analysis,
                dcm_analyze,
                graphql,
                codeowners,
                verbose,
            ),
        },
    };

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
