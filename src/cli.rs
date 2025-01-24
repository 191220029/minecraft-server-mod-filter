use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    FiltServerMod(FiltServerMods),
    PickMods(PickMods),
    CmpMods(CmpModpacks),
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct FiltServerMods {
    /// Path to manifest file
    #[arg(short, long)]
    pub(crate) manifest: PathBuf,
    /// Path to output server & client mod html directory
    #[arg(short, long)]
    pub(crate) out: PathBuf,
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct PickMods {
    /// Path to manifest file
    #[arg(short, long)]
    pub(crate) manifest: PathBuf,
    /// Path to jars' src directory
    #[arg(short, long)]
    pub(crate) jar_src_dir: PathBuf,
    /// Path to jars' dst directory
    #[arg(short, long)]
    pub(crate) jar_dst_dir: PathBuf,
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct CmpModpacks {
    /// Path to package A's modlist
    #[arg(short, long)]
    pub(crate) html_a: PathBuf,
    /// Path to package B's modlist
    #[arg(short, long)]
    pub(crate) html_b: PathBuf,
}
