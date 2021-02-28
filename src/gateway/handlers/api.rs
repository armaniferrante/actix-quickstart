use crate::backend::Backend;
use crate::error::Error;
use crate::store::user::{CreateUser, UpdateUser, User};
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, put, HttpResponse};

#[get("/api/v0/health")]
pub(crate) async fn health(_backend: Data<Backend>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[post("/api/v0/user")]
pub(crate) async fn create_user(
    backend: Data<Backend>,
    user: Json<CreateUser>,
) -> Result<Json<User>, Error> {
    Ok(Json(backend.create_user(&*user).await?))
}

#[get("/api/v0/users")]
pub(crate) async fn all_users(backend: Data<Backend>) -> Result<Json<Vec<User>>, Error> {
    let users = backend.all_users().await?;
    Ok(Json(users))
}

#[get("/api/v0/user/{id}")]
pub(crate) async fn read_user(backend: Data<Backend>, id: Path<i32>) -> Result<Json<User>, Error> {
    Ok(Json(backend.read_user(*id).await?))
}

#[put("/api/v0/user")]
pub(crate) async fn update_user(
    backend: Data<Backend>,
    user: Json<UpdateUser>,
) -> Result<Json<User>, Error> {
    Ok(Json(backend.update_user(&*user).await?))
}
