use std::path::{Path, PathBuf};

/// Checks if the given directory is in a Dart package.
pub fn is_dart_package<P: AsRef<Path>>(cwd: Option<P>) -> bool {
    let path = cwd
        .as_ref()
        .map(|p| p.as_ref())
        .unwrap_or_else(|| Path::new("."));

    find_dart_package_root(Some(path)).is_some()
}

/// Finds the root directory of the Dart package.
pub fn find_dart_package_root<P: AsRef<Path>>(cwd: Option<P>) -> Option<PathBuf> {
    let mut path = cwd
        .as_ref()
        .map(|p| p.as_ref().to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    if !path.is_absolute() {
        path = std::env::current_dir().ok()?.join(&path);
    }

    loop {
        let pubspec = path.join("pubspec.yaml");
        if pubspec.exists() {
            return Some(path);
        }

        if !path.pop() {
            return None;
        }
    }
}

/// Finds the package root containing a specific file.
pub fn find_file_package_root<P: AsRef<Path>>(file_path: P) -> Option<PathBuf> {
    let path = file_path.as_ref();

    if let Some(parent) = path.parent() {
        find_dart_package_root(Some(parent))
    } else {
        None
    }
}

/// Common Dart codegen suffixes to filter out.
pub const COMMON_DART_CODEGEN_SUFFIXES: &[&str] =
    &[".g.dart", ".freezed.dart", ".gr.dart", ".gql.dart"];

/// Filters Dart files that are generated.
pub fn is_generated_dart_file(file: &str) -> bool {
    COMMON_DART_CODEGEN_SUFFIXES
        .iter()
        .any(|suffix| file.ends_with(suffix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_generated_dart_file() {
        assert!(is_generated_dart_file("model.g.dart"));
        assert!(is_generated_dart_file("freezed.freezed.dart"));
        assert!(!is_generated_dart_file("normal.dart"));
    }
}
