use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("There was an internal error")]
    InternalError,
    #[error("An invalid password was given")]
    InvalidPassword,
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
