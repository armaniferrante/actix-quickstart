use crate::backend::Backend;
use crate::store::user::{CreateUser, UpdateUser};
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, put, HttpResponse, Responder};

#[get("/api/v0/health")]
pub(crate) async fn health(_backend: Data<Backend>) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/api/v0/user")]
pub(crate) async fn create_user(backend: Data<Backend>, user: Json<CreateUser>) -> impl Responder {
    Json(backend.create_user(&*user).await.unwrap())
}

#[get("/api/v0/users")]
pub(crate) async fn all_users(backend: Data<Backend>) -> impl Responder {
    let users = backend.all_users().await.unwrap();
    Json(users)
}

#[get("/api/v0/user/{id}")]
pub(crate) async fn read_user(backend: Data<Backend>, id: Path<i32>) -> impl Responder {
    Json(backend.read_user(*id).await.unwrap())
}

#[put("/api/v0/user")]
pub(crate) async fn update_user(backend: Data<Backend>, user: Json<UpdateUser>) -> impl Responder {
    Json(backend.update_user(&*user).await.unwrap())
}
