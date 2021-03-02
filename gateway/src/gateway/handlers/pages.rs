use crate::backend::Backend;
use actix_files::NamedFile;
use actix_web::web::Data;
use actix_web::{get, Responder, Result};
use std::path::PathBuf;

#[get("/")]
pub(crate) async fn index(_backend: Data<Backend>) -> impl Responder {
    react_app()
}

#[get("/login")]
pub(crate) async fn login(_backend: Data<Backend>) -> impl Responder {
    react_app()
}

#[get("/signup")]
pub(crate) async fn signup(_backend: Data<Backend>) -> impl Responder {
    react_app()
}

#[get("/page1")]
pub(crate) async fn page1(_backend: Data<Backend>) -> impl Responder {
    react_app()
}

#[get("/page2")]
pub(crate) async fn page2(_backend: Data<Backend>) -> impl Responder {
    react_app()
}

fn react_app() -> Result<NamedFile> {
    let path = PathBuf::from("./app/build/index.html");
    Ok(NamedFile::open(path)?)
}
