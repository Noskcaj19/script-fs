/// Defines structure for the `.sfs.toml` files and the root config
use serde_derive::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct RootConfig {
    pub name: String,
    pub mount: String,
}

#[derive(Deserialize, Debug)]
pub struct Script {
    pub src: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum File {
    Script { script: Script },
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum FileConfig {
    File { file: File },
}
