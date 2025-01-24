use clap::Parser;
use cli::Args;
use furse::Furse;
use parse::manifest::CFManiestParser;
use verifier::mcmode::McModeVerifier;

mod cli;
mod cmp;
mod jar;
mod manifest;
mod module;
mod parse;
mod utils;
mod verifier;

const API_KEY: &str = "$2a$10$J53vEd4GnG2RXZUZeWLcc.fVY/6/cqB7rxuPri93XLf67XHSATpkG";

fn main() {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    let args = Args::parse();
    let curse_forge = Furse::new(API_KEY);

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
            cmp::entry(cmp_mods, &curse_forge);
        }
    }
}
