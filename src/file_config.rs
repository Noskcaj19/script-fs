/// Defines structure for the `.sfs.toml` files and the root config
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct RootConfig {
    pub name: String,
    pub mount: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Script {
    pub src: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum File {
    Script { script: Script },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum FileConfig {
    File { file: File },
}
