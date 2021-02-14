use crate::common::logging;
use anyhow::Result;
use slog::info;
use std::convert::Into;
use tokio_postgres::NoTls;

pub type Config = deadpool_postgres::Config;

pub fn start(cfg: Config) -> Result<deadpool_postgres::Pool> {
    let logger = logging::get_logger("db");
    info!(logger, "Creating postgres connection pool");
    cfg.create_pool(NoTls).map_err(Into::into)
}
