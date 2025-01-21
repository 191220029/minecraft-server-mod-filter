use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to manifest file
    #[arg(short, long)]
    pub(crate) manifest: PathBuf,
    /// Path to output server & client mod html directory
    #[arg(short, long)]
    pub(crate) out: PathBuf,
}
