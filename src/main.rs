use clap::Parser;
use cli::Args;
use parse::manifest::CFManiestParser;
use verifier::mcmode::McModeVerifier;

mod cli;
mod module;
mod parse;
mod utils;
mod verifier;

fn main() {
    env_logger::init();
    let args = Args::parse();
    let mut cf_parser = CFManiestParser::new(&args.manifest).unwrap();
    cf_parser.pull(McModeVerifier);
    cf_parser.write_curse_forge_manifest(&args.out).unwrap();
}
