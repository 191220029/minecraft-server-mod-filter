use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use furse::Furse;
use serde::Deserialize;

use crate::utils::cli_pause;

pub struct ManifestParser;
impl ManifestParser {
    pub fn parse(&self, pth: &Path) -> Manifest {
        let f = match File::open(pth) {
            Ok(f) => f,
            Err(e) => {
                log::error!("Fail to open {}. Error: {}", pth.to_str().unwrap(), e);
                cli_pause(1);
            }
        };
        let mut reader = BufReader::new(f);
        let mut buf = vec![];
        match reader.read_to_end(&mut buf) {
            Ok(_) => (),
            Err(e) => {
                log::error!("Fail to read {}. Error: {}", pth.to_str().unwrap(), e);
                cli_pause(1);
            }
        }
        match serde_json::from_str(&String::from_utf8(buf).unwrap()) {
            Ok(m) => {
                log::info!("Succeed to parse {}.", pth.to_str().unwrap());
                m
            }
            Err(e) => {
                log::error!("Fail to parse {}. Error: {}", pth.to_str().unwrap(), e);
                cli_pause(1);
            }
        }
    }
}

#[derive(Deserialize)]
#[allow(non_snake_case, unused)]
pub struct Manifest {
    pub(crate) minecraft: Minecraft,
    pub(crate) manifestType: String,
    pub(crate) manifestVersion: u32,
    pub(crate) name: String,
    pub(crate) author: String,
    pub(crate) files: Vec<ModFile>,
    pub(crate) overrides: String,
}

#[derive(Deserialize)]
#[allow(non_snake_case, unused)]
pub struct Minecraft {
    pub(crate) version: String,
    pub(crate) modLoaders: Vec<ModLoader>,
}

#[derive(Deserialize)]
#[allow(non_snake_case, unused)]
pub struct ModLoader {
    pub(crate) id: String,
    pub(crate) primary: bool,
}

#[derive(Deserialize, Hash, PartialEq, Eq)]
#[allow(non_snake_case, unused)]
pub struct ModFile {
    pub(crate) projectID: u128,
    pub(crate) fileID: u128,
    pub(crate) required: bool,
}

impl ModFile {
    /// visit api.curseforge to get modfile's detailed info
    pub async fn pull_mod(&self, curse_forge: &Furse) -> furse::structures::mod_structs::Mod {
        match curse_forge.get_mod(self.projectID as i32).await {
            Ok(m) => m,
            Err(e) => {
                log::error!(
                    "Fail to pull mod {} from curseforge. Error: {}",
                    self.projectID,
                    e
                );
                cli_pause(1);
            }
        }
    }
}

#[cfg(test)]
mod test_manifest {
    use std::{env, path::Path};

    use super::ManifestParser;

    const MANIFEST_FILE: &str = "tests/manifest.json";

    #[test]
    #[ignore]
    fn test_deserde() {
        env::set_var("RUST_LOG", "info");
        env_logger::init();
        let _ = ManifestParser.parse(&Path::new(MANIFEST_FILE));
    }
}
