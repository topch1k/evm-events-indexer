use clap::Parser;
use serde::Serialize;
use std::path::PathBuf;
use twelf::{Layer, config};

use crate::{errors::IndexerResult, event::EventIndexingInfo};

#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
    pub config_path: String,
}

pub fn parse() -> Args {
    Args::parse()
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
#[config]
pub struct Config {
    pub log_level: String, //TODO: Use log level from config
    pub port: u32,
    pub event_info: EventIndexingInfo,
    pub db_path: String,
    pub node_url: String,
}

pub fn load(path: PathBuf) -> IndexerResult<Config> {
    let conf = Config::with_layers(&[Layer::Yaml(path)])?; //TODO: Add additional level: ENV

    Ok(conf)
}
