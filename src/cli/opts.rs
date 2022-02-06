use std::path::PathBuf;

#[derive(Debug, clap::Parser, derive_getters::Getters)]
#[clap(name = "word-guessing-game-api", version)]
pub(crate) struct Opts {
    /// Set config file path
    #[clap(short, long)]
    config_path: PathBuf,
}
