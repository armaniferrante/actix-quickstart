use actix_web::error::ResponseError;
use actix_web::http::{header, StatusCode};
use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("There was an internal error")]
    InternalError,
    #[error("An invalid password was given")]
    InvalidPassword,
    #[error("Auth checker not properly configured")]
    InvalidAuthConfig,
    #[error("Invalid JWT")]
    InvalidJwt,
    #[error("Refresh token expired")]
    ExpiredRefreshToken,
    #[error("The user doesn't have authorization to perform this action.")]
    Unauthorized,
    #[error("An unknown error occured")]
    Unknown(#[from] anyhow::Error),
}

impl From<rand::Error> for Error {
    fn from(_e: rand::Error) -> Error {
        Error::InternalError
    }
}

impl From<argon2::Error> for Error {
    fn from(_e: argon2::Error) -> Error {
        Error::InternalError
    }
}

/// Maps domain errors into something usable by REST APIs
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Error::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidAuthConfig { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InvalidJwt => StatusCode::UNAUTHORIZED,
            Error::InvalidPassword => StatusCode::UNAUTHORIZED,
            Error::ExpiredRefreshToken => StatusCode::UNAUTHORIZED,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR, // todo
        }
    }
}
