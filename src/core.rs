use crate::file_config::{FileConfig, RootConfig};
use indexmap::IndexMap;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug)]
pub(crate) enum CoreFile {
    File(FileConfig),
    Dir(Vec<usize>),
}

/// Internal data store for the filesystem
#[derive(Debug)]
pub struct Core {
    pub root_path: PathBuf,
    pub root_config: RootConfig,
    pub(crate) entries: IndexMap<PathBuf, CoreFile>,
}

impl Core {
    /// Traverses a path and populates the file core
    pub fn new_from_path(path: PathBuf) -> Core {
        // TODO: Error handling
        let root_config = load_root_config(path.as_path()).expect("No root config file");
        let files =
            find_config_paths(path.as_path()).filter_map(|p| match load_config(p.as_path()) {
                Ok(c) => Some((p, c)),
                Err(e) => {
                    eprintln!("Error loading {:?}: {:?}", p, e);
                    None
                }
            });

        let mut entries: IndexMap<PathBuf, CoreFile> = IndexMap::new();
        for (path, entry) in files {
            if let Some(parent) = path.as_path().parent() {
                entries.entry(path.clone()).or_insert(CoreFile::File(entry));
                // TODO: Entry api that gets index?
                // TODO: Remove second lookup
                if let Some((index, _, _)) = entries.get_full(&path) {
                    let dir = entries
                        .entry(parent.to_owned())
                        .or_insert(CoreFile::Dir(vec![]));
                    if let CoreFile::Dir(ref mut files) = dir {
                        files.push(index);
                    }
                }
            } else {
                // TODO: ?
            }
        }

        Core {
            root_path: path,
            root_config,
            entries,
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
