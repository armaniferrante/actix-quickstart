use crate::backend::Backend;
use crate::store::user::User;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, put, Responder};

#[post("/api/v0/user")]
pub(crate) async fn create_user(backend: Data<Backend>) -> impl Responder {
    Json(backend.create_user().await.unwrap())
}

#[get("/api/v0/users")]
pub(crate) async fn all_users(backend: Data<Backend>) -> impl Responder {
    let users = backend.all_users().await.unwrap();
    println!("users: {:?}", users);
    Json(users)
}

#[get("/api/v0/user/{id}")]
pub(crate) async fn read_user(backend: Data<Backend>, id: Path<i32>) -> impl Responder {
    Json(backend.read_user(*id).await.unwrap())
}

#[put("/api/v0/user")]
pub(crate) async fn update_user(backend: Data<Backend>, user: Json<User>) -> impl Responder {
    Json(backend.update_user(&*user).await.unwrap())
}
