use crate::common::auth::AuthTokens;
use crate::error::Error;
use actix_http::Payload;
use actix_web::HttpMessage;
use actix_web::{Error as ActixError, FromRequest, HttpRequest};
use futures::future::{self, FutureExt};
use std::future::Future;
use std::pin::Pin;

pub struct Auth {
    pub tokens: AuthTokens,
}

impl FromRequest for Auth {
    type Config = ();
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, ActixError>>>>;

    #[inline]
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(cookie) = req.cookie("JWT") {
            let access = cookie.value().to_string();
            if let Some(cookie) = req.cookie("REFRESH") {
                let refresh = cookie.value().to_string();
                return Box::pin(async move {
                    Ok(Auth {
                        tokens: AuthTokens { access, refresh },
                    })
                });
            }
        }
        future::err(Error::InvalidAuthConfig.into()).boxed_local()
    }
}
