use crate::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

const DELETE_MUST_CONTAINS_ANY_OF: &[&str; 1] = &[".git"];

/// Rudementary hardcoded way to check if a path seems to be safe to be deleted.
/// Designed to be application specific.
/// (far from perfect, but better than nothing)
pub fn safer_remove_dir_all(dir: &Path) -> Result<()> {
    let dir_str = dir.to_string_lossy();

    DELETE_MUST_CONTAINS_ANY_OF
        .iter()
        .find(|v| dir_str.contains(*v))
        .ok_or_else(|| Error::PathNotSafeToDelete(s!(dir_str)))?;

    // TODO: Make error more informative
    fs::remove_dir_all(dir)?;
    Ok(())
}

/// Create a new PathBuf from a root and a '/' delimited components
#[allow(dead_code)]
pub fn join_paths(root: &Path, sub_path: &str) -> PathBuf {
    let parts = sub_path.split('/');
    let mut path = root.to_owned();
    for part in parts {
        path.push(part)
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn join_paths_joins_accurately() {
        let root = Path::new("/home/user");
        let sub_path = "foo/bar/baz";
        let expected = Path::new("/home/user/foo/bar/baz");
        let actual = join_paths(root, sub_path);
        assert_eq!(actual, expected);
    }
}
