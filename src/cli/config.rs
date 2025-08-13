use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;
use twelf::{Layer, config};

use crate::{cli::commands::Commands, errors::IndexerResult, event::EventIndexingInfo};

#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(long)]
    pub config_path: String,
}

pub fn parse() -> Args {
    Args::parse()
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
#[config]
pub struct Config {
    pub event_info: EventIndexingInfo,
    pub db_path: String,
    pub node_url: String,
}

pub fn load(path: PathBuf) -> IndexerResult<Config> {
    let conf = Config::with_layers(&[Layer::Yaml(path)])?;
    Ok(conf)
}
