use std::{collections::HashSet, path::PathBuf};

use fs::find_jars;

use crate::{cli::PickMods, parse::manifest::CFManiestParser};

mod fs;

struct JarLocal {
    name: String,
    pth: PathBuf,
}

impl PickMods {
    pub fn entry(&self) {
        let local_jars = find_jars(&self.jar_src_dir).unwrap();
        let manifest_parser = CFManiestParser::new(&self.manifest).unwrap();

        // let jar_pool = manifest_parser.modules.into_iter().map(|module| module.)

        // local_jars.into_iter().filter(|jar| )
    }
}

impl From<PathBuf> for JarLocal {
    fn from(value: PathBuf) -> Self {
        Self {
            name: value.file_name().unwrap().to_str().unwrap().to_string(),
            pth: value,
        }
    }
}
