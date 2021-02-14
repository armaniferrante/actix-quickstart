use crate::common::logging;
use anyhow::Result;
use serde::Deserialize;
use slog::info;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::convert::Into;

#[derive(Debug, Deserialize)]
pub struct Config {
    host: String,
    port: String,
    user: String,
    password: String,
    database: String,
}

impl Config {
    pub fn to_url(&self) -> String {
        format!(
            "postgres://{0}:{1}@{2}/{3}",
            self.user, self.password, self.host, self.database
        )
    }
}

pub async fn start(cfg: Config) -> Result<PgPool> {
    let logger = logging::get_logger("db");
    info!(logger, "Creating postgres connection pool");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&cfg.to_url())
        .await
        .map_err(Into::into)
}
