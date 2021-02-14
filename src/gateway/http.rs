use crate::backend::Backend;
use crate::gateway::handlers;
use crate::store::Store;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use serde::Deserialize;
use std::io::Result;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub port: u16,
}

pub(crate) async fn start(cfg: Config, store: Store) -> Result<()> {
    let backend = Backend::new(store);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(backend.clone())
            .service(handlers::read_user)
            .service(handlers::create_user)
            .service(handlers::update_user)
            .service(handlers::all_users)
    })
    .bind(("127.0.0.1", cfg.port))?
    .run()
    .await
}
