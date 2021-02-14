use crate::common::{db, logging};
use crate::store::Store;
use clap::Clap;
use config::{Config as ConfigParser, ConfigError, Environment, File};
use serde::Deserialize;
use std::path::Path;

pub(crate) mod handlers;
pub(crate) mod http;

#[derive(Clap)]
struct Opts {
    #[clap(long)]
    config: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    http: http::Config,
    logging: logging::Config,
    db: db::Config,
}

impl Config {
    fn from_env() -> Result<Self, ConfigError> {
        let opts = Opts::parse();
        let mut cfg = ConfigParser::new();
        cfg.merge(Environment::new().separator("_"))?;
        cfg.merge(File::from(Path::new(&opts.config)))?;
        cfg.try_into()
    }
}

pub async fn start() -> std::io::Result<()> {
    let cfg = Config::from_env().unwrap();

    logging::start(cfg.logging);
    let pool = db::start(cfg.db).unwrap();
    let store = Store::new(pool);
    http::start(cfg.http, store).await?;

    Ok(())
}
