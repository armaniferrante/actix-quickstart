use crate::backend::Backend;
use crate::error::Error;
use crate::gateway::extractors::auth::Auth;
use crate::gateway::extractors::user_id::UserId;
use actix_web::cookie::Cookie;
use actix_web::web::{Data, Json};
use actix_web::{get, post, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[post("/auth/v0/login")]
pub(crate) async fn login(backend: Data<Backend>, req: Json<Login>) -> Result<HttpResponse, Error> {
    let new_tokens = backend.login(&req.username, &req.password).await?;
    let resp = HttpResponse::Ok()
        .cookie(
            Cookie::build("JWT", new_tokens.access.clone())
                .http_only(true)
                .secure(backend.cfg.secure)
                .finish(),
        )
        .cookie(
            Cookie::build("REFRESH", new_tokens.access.clone())
                .http_only(true)
                .secure(backend.cfg.secure)
                .finish(),
        )
        .finish();
    Ok(resp)
}

#[get("/auth/v0/logout")]
pub(crate) async fn logout(
    backend: Data<Backend>,
    _user_id: UserId,
    auth: Auth,
) -> Result<HttpResponse, Error> {
    backend.logout(auth.tokens).await?;
    let resp = HttpResponse::Ok()
        .cookie(
            Cookie::build("JWT", "")
                .http_only(true)
                .secure(backend.cfg.secure)
                .finish(),
        )
        .cookie(
            Cookie::build("REFRESH", "")
                .http_only(true)
                .secure(backend.cfg.secure)
                .finish(),
        )
        .finish();
    Ok(resp)
}

#[post("/auth/v0/refresh")]
pub(crate) async fn refresh(
    backend: Data<Backend>,
    user_id: UserId,
    auth: Auth,
) -> Result<HttpResponse, Error> {
    let new_tokens = backend.refresh(user_id.id, auth.tokens).await?;
    let resp = HttpResponse::Ok()
        .cookie(
            Cookie::build("JWT", new_tokens.access.clone())
                .http_only(true)
                .secure(backend.cfg.secure)
                .finish(),
        )
        .cookie(
            Cookie::build("REFRESH", new_tokens.access.clone())
                .http_only(true)
                .secure(backend.cfg.secure)
                .finish(),
        )
        .finish();
    Ok(resp)
}
