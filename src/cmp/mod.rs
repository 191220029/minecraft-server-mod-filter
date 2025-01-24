use std::collections::HashSet;

use crate::{cli::CmpModpacks, module::Module, parse::manifest::CFManiestParser, utils::cli_pause};

pub fn entry(args: CmpModpacks) {
    match args.cmp() {
        Ok(_) => (),
        Err(e) => log::error!("Fail to compare packs. Error info: {}", e),
    };
    cli_pause();
}

impl CmpModpacks {
    fn cmp(&self) -> Result<(), std::io::Error> {
        let mut pack_a = CFManiestParser::new(&self.html_a)?
            .modules
            .into_iter()
            .collect::<HashSet<Module>>();
        log::info!("succeed to parse modpack {}", self.html_a.to_str().unwrap());
        let pack_b = CFManiestParser::new(&self.html_b)?.modules;
        log::info!("succeed to parse modpack {}", self.html_b.to_str().unwrap());

        let mut a_missing = vec![];
        for m in pack_b {
            if !pack_a.contains(&m) {
                a_missing.push(m);
            } else {
                pack_a.remove(&m);
            }
        }

        if a_missing.len() > 0 {
            println!(
                "Pack {} Missing:\n\t {}",
                self.html_a.to_str().unwrap(),
                a_missing
                    .iter()
                    .enumerate()
                    .map(|(num, module)| format!("{} {}", num, module.link))
                    .collect::<Vec<String>>()
                    .join("\n\t")
            );
        }
        if pack_a.len() > 0 {
            println!(
                "Pack {} Missing:\n\t {}",
                self.html_b.to_str().unwrap(),
                pack_a
                    .iter()
                    .enumerate()
                    .map(|(num, module)| format!("{} {}", num, module.link))
                    .collect::<Vec<String>>()
                    .join("\n\t")
            );
        }
        if a_missing.len() + pack_a.len() == 0 {
            println!("The two packs having the same modlist.")
        }

        Ok(())
    }
}
