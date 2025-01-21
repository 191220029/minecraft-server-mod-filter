use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use futures::future::join_all;
use log::info;
use regex::Regex;

use crate::{
    module::{Module, ServerFlag},
    verifier::Verifier,
};

pub struct CFManiestParser {
    /// Path to manifest file
    pth: PathBuf,
    modules: Vec<Module>,
}

impl CFManiestParser {
    pub fn new(pth: &Path) -> Result<Self, io::Error> {
        let mut parser = Self {
            pth: pth.to_path_buf(),
            modules: vec![],
        };
        parser.parse_curse_forge_manifest()?;
        Ok(parser)
    }

    /// Get mod info from the given `verifier`.
    pub fn pull(&mut self, verifier: impl Verifier) {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            join_all(self.modules.iter_mut().map(|module| async {
                verifier.verify(module).await;
            }))
            .await
        });
    }

    fn parse_curse_forge_manifest(&mut self) -> Result<(), io::Error> {
        let mut reader = BufReader::new(File::open(&self.pth)?);
        let re = Regex::new(
            r#"<a href="https://www\.curseforge\.com/minecraft/[^"]+">([^<(]+)([^<])*</a>"#,
        )
        .unwrap();

        let mut buf = String::new();
        while reader.read_line(&mut buf)? > 0 {
            if let Some(cpt) = re.captures(&buf) {
                info!(
                    "Discover mod from manifest: {:?}",
                    cpt.get(1).unwrap().as_str().trim()
                );
                self.modules
                    .push(Module::new(cpt.get(1).unwrap().as_str().trim(), &buf))
            } else {
                info!("Failed to capture mod name from: {:?}", buf);
            }
            buf.clear();
        }
        Ok(())
    }

    /// Write curse forge manifest file to `out_dir`, where `out_dir/server-manifest.html` contains all the server needed mods,
    /// `out_dir/client-manifest.html` contains all the mods,
    pub fn write_curse_forge_manifest(&self, out_dir: &Path) -> Result<(), io::Error> {
        let header = "<ul>\n";
        let tail = "</ul>\n";

        let client_pth = out_dir.join("client-manifest.html");
        let f = File::create(&client_pth)?;
        let mut writer = BufWriter::new(f);
        writer.write_fmt(format_args!(
            "{}{}{}",
            header,
            self.modules
                .iter()
                .map(|module| module.raw.as_str())
                .collect::<Vec<&str>>()
                .join("\n"),
            tail
        ))?;
        info!("Write all modules to {}", client_pth.display());

        let server_pth = out_dir.join("server-manifest.html");
        let f = File::create(&server_pth)?;
        let mut writer = BufWriter::new(f);
        writer.write_fmt(format_args!(
            "{}{}{}",
            header,
            self.modules
                .iter()
                .filter_map(|module| if module.server_flag == ServerFlag::ServerNeeded {
                    Some(module.raw.as_str())
                } else {
                    None
                })
                .collect::<Vec<&str>>()
                .join("\n"),
            tail
        ))?;
        info!("Write server modules to {}", server_pth.display());

        Ok(())
    }
}

#[cfg(test)]
mod test_cf_manifest {
    use std::path::PathBuf;

    use crate::parse::manifest::CFManiestParser;

    #[test]
    fn test_cf_manifest_parser() {
        let parser = CFManiestParser::new(&PathBuf::from("tests/curseforge.html")).unwrap();
        assert_eq!(parser.modules.len(), 215)
    }
}
