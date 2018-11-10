use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

/// Internal data store for the filesystem
#[derive(Debug)]
pub struct Core {
    root: PathBuf,
    entries: HashMap<PathBuf, ()>,
}

impl Core {
    /// Traverses a path and populates the file core
    pub fn new_from_path(path: PathBuf) -> Core {
        let entries = find_config_paths(path.as_path()).zip(std::iter::repeat(()));
        Core {
            root: path,
            entries: entries.collect(),
        }
    }
}

/// Find all config files
fn find_config_paths(path: &Path) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(&path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|f| f.ends_with(".sfs.toml"))
                .unwrap_or(false)
        })
        .map(|f| f.into_path())
}
