use crate::utils::shell::is_command_installed;
use anyhow::Result;

pub fn externals(verbose: bool) -> Result<()> {
    let commands = vec![
        ("dart", "Dart SDK"),
        ("dcm", "DCM"),
        ("melos", "Melos"),
        ("claude", "Claude CLI"),
    ];

    let mut all_installed = true;

    for (cmd, name) in &commands {
        let installed = is_command_installed(cmd);

        if verbose {
            if installed {
                eprintln!("✓ {} is installed", name);
            } else {
                eprintln!("✗ {} is not installed", name);
            }
        }

        if !installed {
            all_installed = false;
            println!("{}", cmd);
        }
    }

    if all_installed {
        if verbose {
            eprintln!("All external dependencies are installed");
        }
        Ok(())
    } else {
        std::process::exit(1);
    }
}

pub fn version(verbose: bool) -> Result<()> {
    // For now, just return success
    // In a full implementation, this would check the latest version on GitHub
    if verbose {
        eprintln!("Version check not yet implemented");
    }
    Ok(())
}
