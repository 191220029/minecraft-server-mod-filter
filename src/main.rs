use clap::Parser;
use cli::Args;
use parse::manifest::CFManiestParser;
use verifier::mcmode::McModeVerifier;

mod cli;
mod cmp;
mod jar;
mod module;
mod parse;
mod utils;
mod verifier;

fn main() {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    let args = Args::parse();

    match args {
        Args::FiltServerMod(filt_server_mods) => {
            let mut cf_parser = CFManiestParser::new(&filt_server_mods.manifest).unwrap();
            cf_parser.pull(McModeVerifier);
            cf_parser
                .write_curse_forge_manifest(&filt_server_mods.out)
                .unwrap();
        }
        Args::PickMods(_) => {
            unimplemented!()
        }
        Args::CmpMods(cmp_mods) => {
            cmp::entry(cmp_mods);
        }
    }
}
