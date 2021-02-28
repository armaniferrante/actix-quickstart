//! Auth middleware transforms a request with a `JWT` cookie into a request
//! with an `Authorization: Bearer` header. This is useful for preprocessing
//! requests from the browser that use HTTP-only and secure cookies, instead
//! of something like a mobile app that would just use the authorization header
//! directly.

use crate::error::Error;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::{Header, IntoHeaderValue};
use actix_web::Error as ActixError;
use actix_web::HttpMessage;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use futures::future::{self, FutureExt, Ready};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct AuthCookies;

impl<S, B> Transform<S> for AuthCookies
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type InitError = ();
    type Transform = AuthCookiesMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(AuthCookiesMiddleware { service })
    }
}

pub struct AuthCookiesMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthCookiesMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut req = req;

        if let Some(cookie) = req.cookie("JWT") {
            let cookie = cookie.value().to_string();
            let head = req.head_mut();

            let bearer = Bearer::new(cookie);
            let auth = Authorization::from(bearer);

            let header_value = match auth.try_into() {
                Err(_e) => return future::err(Error::InvalidJwt.into()).boxed_local(),
                Ok(hv) => hv,
            };
            head.headers
                .insert(Authorization::<Bearer>::name(), header_value);
        }

        Box::pin(self.service.call(req))
    }
}
