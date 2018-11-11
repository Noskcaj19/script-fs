use crate::file_config::{FileConfig, RootConfig};
use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

/// Internal data store for the filesystem
#[derive(Debug)]
pub struct Core {
    root_path: PathBuf,
    root_config: RootConfig,
    entries: HashMap<PathBuf, FileConfig>,
}

impl Core {
    /// Traverses a path and populates the file core
    pub fn new_from_path(path: PathBuf) -> Core {
        // TODO: Error handling
        let root_config = load_root_config(path.as_path()).expect("No root config file");
        let entries =
            find_config_paths(path.as_path()).filter_map(|p| match load_config(p.as_path()) {
                Ok(c) => Some((p, c)),
                Err(e) => {
                    eprintln!("Error loading {:?}: {:?}", p, e);
                    None
                }
            });
        Core {
            root_path: path,
            root_config,
            entries: entries.collect(),
        }
    }
}

/// Load the root config from a template path
// TODO: Error handling
fn load_root_config(path: &Path) -> Result<RootConfig, ()> {
    let mut file = File::open(path.join("sfs.toml")).map_err(|_| ())?;
    let mut input = String::new();
    file.read_to_string(&mut input).map_err(|_| ())?;

    toml::from_str(&input).map_err(|_| ())
}

fn load_config(path: &Path) -> Result<FileConfig, ()> {
    let mut file = File::open(path).map_err(|_| ())?;
    let mut input = String::new();
    file.read_to_string(&mut input).map_err(|_| ())?;

    toml::from_str(&input).map_err(|_| ())
}

/// Find all config files
// TODO: Error handling (less important)
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
