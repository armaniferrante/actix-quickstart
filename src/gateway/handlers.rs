use crate::backend::Backend;
use crate::store::user::{CreateUser, User};
use actix_files::NamedFile;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, put, HttpResponse, Responder, Result};
use std::path::PathBuf;

pub mod api {
    use super::*;

    #[get("/api/v0/health")]
    pub(crate) async fn health(backend: Data<Backend>) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[post("/api/v0/user")]
    pub(crate) async fn create_user(
        backend: Data<Backend>,
        user: Json<CreateUser>,
    ) -> impl Responder {
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
    pub(crate) async fn update_user(backend: Data<Backend>, user: Json<User>) -> impl Responder {
        Json(backend.update_user(&*user).await.unwrap())
    }
}

pub mod pages {
    use super::*;

    #[get("/")]
    pub(crate) async fn index(backend: Data<Backend>) -> impl Responder {
        react_app()
    }

    #[get("/page1")]
    pub(crate) async fn page1(backend: Data<Backend>) -> impl Responder {
        react_app()
    }

    #[get("/page2")]
    pub(crate) async fn page2(backend: Data<Backend>) -> impl Responder {
        react_app()
    }

    fn react_app() -> Result<NamedFile> {
        let path = PathBuf::from("./app/build/index.html");
        Ok(NamedFile::open(path)?)
    }
}
