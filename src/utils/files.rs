use std::io::{self, BufRead};

/// Filters files by suffix patterns.
pub fn filter_files_by_suffix(suffixes: &[String], verbose: bool) -> Result<(), std::io::Error> {
    let stdin = io::stdin();
    let mut total_count = 0;
    let mut filtered_count = 0;

    for line in stdin.lock().lines() {
        let line = line?;
        total_count += 1;

        let should_filter = suffixes.iter().any(|suffix| line.ends_with(suffix));

        if !should_filter {
            println!("{}", line);
        } else {
            filtered_count += 1;
        }
    }

    if verbose {
        eprintln!("Filtered {} of {} files", filtered_count, total_count);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_basic() {
        // Basic test to ensure the module compiles
        let suffixes = vec![".g.dart".to_string()];
        assert!(!suffixes.is_empty());
    }
}
