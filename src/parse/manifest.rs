use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use log::info;
use regex::Regex;

use crate::module::Module;

pub fn parse_curse_forge_manifest(pth: &Path) -> Result<Vec<Module>, io::Error> {
    let mut modules = vec![];

    let mut reader = BufReader::new(File::open(pth)?);
    let re = Regex::new(
        r#"<a href="https://www\.curseforge\.com/minecraft/mc-mods/[^"]+">([^<(]+)([^<])*</a>"#,
    )
    .unwrap();

    let mut buf = String::new();
    while reader.read_line(&mut buf)? > 0 {
        if let Some(cpt) = re.captures(&buf) {
            info!("{:?}", cpt.get(1).unwrap().as_str().trim());
            modules.push(Module::new(cpt.get(1).unwrap().as_str().trim()))
        } else {
            info!("Failed to capture mod name from: {:?}", buf);
        }
        buf.clear();
    }

    Ok(modules)
}

#[cfg(test)]
mod test_cf_manifest {
    use std::path::PathBuf;

    use super::parse_curse_forge_manifest;

    #[test]
    fn test_cf_manifest() {
        env_logger::init();
        parse_curse_forge_manifest(&PathBuf::from("tests/curseforge.html")).unwrap();
        // assert_eq!(parse_curse_forge_manifest(&PathBuf::from("tests/curseforge.html")).unwrap().len(), 214);
    }
}
