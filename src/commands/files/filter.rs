use crate::utils::files::filter_files_by_suffix;
use anyhow::Result;

pub fn suffix(suffixes: Vec<String>, verbose: bool) -> Result<()> {
    filter_files_by_suffix(&suffixes, verbose)?;
    Ok(())
}
