use crate::backend::Backend;
use crate::common::auth::Auth;
use crate::gateway::handlers;
use crate::store::Store;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use handlers::{api, auth, pages};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub port: u16,
}

pub(crate) async fn start(cfg: Config, auth: Auth, store: Store) -> std::io::Result<()> {
    let backend = Backend::new(auth, store);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(backend.clone())
            .service(api::health)
            .service(api::create_user)
            .service(api::read_user)
            .service(api::update_user)
            .service(api::all_users)
            .service(auth::login)
            .service(auth::logout)
            .service(pages::index)
            .service(pages::signup)
            .service(pages::login)
            .service(pages::page1)
            .service(pages::page2)
            .service(Files::new("/static", "./app/build/static"))
            .service(Files::new("/", "./app/public").index_file("index.html"))
    })
    .bind(("127.0.0.1", cfg.port))?
    .run()
    .await
}
