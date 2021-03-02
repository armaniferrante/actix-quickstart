use crate::common::auth::Auth;
use crate::error::Error;
use actix_http::{http::header::Header, Payload};
use actix_web::{Error as ActixError, FromRequest, HttpRequest};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use futures::future::{self, FutureExt};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug)]
pub struct UserId {
    pub id: i32,
}

impl FromRequest for UserId {
    type Config = UserIdConfig;
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, ActixError>>>>;

    #[inline]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let config = req.app_data::<Self::Config>();
        match config {
            Some(config) => {
                let auth = config.auth.clone();
                match bearer_token(req) {
                    Ok(token) => Box::pin(async move {
                        let decoded = auth.decode(&token)?.claims;
                        let id = decoded.sub;
                        Ok(UserId { id })
                    }),
                    Err(error) => future::err(error).boxed_local(),
                }
            }
            None => future::err(Error::InvalidAuthConfig.into()).boxed_local(),
        }
    }
}

#[derive(Clone, Default)]
pub struct UserIdConfig {
    pub auth: Auth,
}

#[inline]
fn bearer_token(req: &HttpRequest) -> Result<String, ActixError> {
    let bearer = Authorization::<Bearer>::parse(req)?;
    Ok(bearer.into_scheme().token().to_string())
}
