use crate::backend::Backend;
use crate::store::user::{CreateUser, User};
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, put, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[post("/auth/v0/login")]
pub(crate) async fn login(backend: Data<Backend>, req: Json<Login>) -> impl Responder {
    let auth_tokens = backend.login(&req.username, &req.password).await.unwrap();
    // * Set auth header / cookies with access + refresh tokens.
    HttpResponse::Ok()
}

#[get("/auth/v0/logout")]
pub(crate) async fn logout(backend: Data<Backend>, req: HttpRequest) -> impl Responder {
    // TODO:
    //
    // * Get refresh token.
    // * Set valid flag to false for the given refresh token and save to DB.
    // * Set cookies to empty.
    //
    // Done.
    HttpResponse::Ok()
}

#[post("/auth/v0/refresh")]
pub(crate) async fn refresh(backend: Data<Backend>, req: HttpRequest) -> impl Responder {
    // TODO:
    //
    // * Get reresh token from cookies or auth headers.
    // * Get refresh token from the database.
    // * Check refresh token is still valid (not expired, not logged out).
    // * Create new access token.
    // * Set cookies to use the new access token.
    //
    // Done.
    HttpResponse::Ok()
}
