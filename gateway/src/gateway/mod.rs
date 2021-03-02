use crate::common::{auth, db, logging};
use crate::store::Store;
use clap::Clap;
use config::{Config as ConfigParser, ConfigError, Environment, File};
use serde::Deserialize;
use std::path::Path;

pub(crate) mod extractors;
pub(crate) mod handlers;
pub(crate) mod http;
pub(crate) mod middleware;

#[derive(Clap)]
struct Opts {
    #[clap(long)]
    config: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    logging: logging::Config,
    http: http::Config,
    db: db::Config,
    auth: auth::Config,
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
    let auth = auth::start(cfg.auth);
    let conn = db::start(cfg.db).await.unwrap();
    http::start(cfg.http, auth, Store::new(conn)).await?;

    Ok(())
}
