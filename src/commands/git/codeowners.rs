use anyhow::Result;

pub fn check(verbose: bool) -> Result<()> {
    if verbose {
        eprintln!("CODEOWNERS check not yet implemented");
    }
    Ok(())
}
