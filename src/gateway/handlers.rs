use crate::backend::Backend;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::{get, post, Responder};

#[get("/person")]
pub(crate) async fn read_person(backend: Data<Backend>) -> impl Responder {
    Json(backend.read_person())
}

#[post("/person")]
pub(crate) async fn create_person(backend: Data<Backend>) -> impl Responder {
    Json(backend.create_person())
}
