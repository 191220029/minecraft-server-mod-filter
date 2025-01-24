use std::collections::HashSet;

use furse::Furse;
use futures::future::join_all;

use crate::{
    cli::CmpModpacks,
    manifest::{ManifestParser, ModFile},
    utils::cli_pause,
};

pub fn entry(args: CmpModpacks, curse_forge: &Furse) {
    match args.cmp(curse_forge) {
        Ok(_) => (),
        Err(e) => log::error!("Fail to compare packs. Error info: {}", e),
    };
    cli_pause(0);
}

impl CmpModpacks {
    fn cmp(&self, curse_forge: &Furse) -> Result<(), std::io::Error> {
        let mut pack_a = ManifestParser
            .parse(&self.manifest_a)
            .files
            .into_iter()
            .collect::<HashSet<ModFile>>();
        log::info!(
            "succeed to parse modpack {}",
            self.manifest_a.to_str().unwrap()
        );
        let pack_b = ManifestParser.parse(&self.manifest_b).files;
        log::info!(
            "succeed to parse modpack {}",
            self.manifest_b.to_str().unwrap()
        );

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
                "Pack {} Missing:\n\t{}",
                self.manifest_a.to_str().unwrap(),
                {
                    a_missing.sort_by(|a, b| a.projectID.cmp(&b.projectID));
                    tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async {
                        join_all(a_missing.iter().enumerate().map(|(num, module)| async move {
                            let module = module.pull_mod(curse_forge).await;
                            format!("{} | {} | {} | {}", num, module.links.website_url, module.name, module.id)
                        }))
                        .await
                    })
                    .join("\n\t")}
            );
        }
        if pack_a.len() > 0 {
            let mut pack_a_remaining = pack_a.into_iter().collect::<Vec<_>>();
            pack_a_remaining.sort_by(|a, b| a.projectID.cmp(&b.projectID));

            println!(
                "Pack {} Missing:\n\t{}",
                self.manifest_b.to_str().unwrap(),
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(async {
                        join_all(pack_a_remaining.iter().enumerate().map(|(num, module)| async move {
                            let module = module.pull_mod(curse_forge).await;
                            format!("{} | {} | {} | {}", num, module.links.website_url, module.name, module.id)
                        }))
                        .await
                    })
                    .join("\n\t")
            );
        } else if a_missing.len() == 0 {
            println!("The two packs having the same modlist.")
        }

        Ok(())
    }
}
