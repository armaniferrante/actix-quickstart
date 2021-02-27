use anyhow::Result;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::convert::Into;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
pub struct Config {
    secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthTokens {
    pub refresh: RefreshToken,
    pub access: AccessToken,
}

pub type RefreshToken = String;
pub type AccessToken = String;

pub fn start(cfg: Config) -> Auth {
    Auth::new(
        "jwt".to_string(),
        Algorithm::HS256,
        cfg.secret,
        Duration::from_secs(60 * 60 * 24),
        Duration::from_secs(60 * 60 * 24),
    )
}

#[derive(Clone)]
pub struct Auth {
    iss: String,
    header: Header,
    validation: Validation,
    access_token_lifetime: Duration,
    refresh_token_lifetime: Duration,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey<'static>,
}

impl Auth {
    pub fn new(
        iss: String,
        alg: Algorithm,
        secret: String,
        access_token_lifetime: Duration,
        refresh_token_lifetime: Duration,
    ) -> Self {
        let header = Header::new(alg);
        let validation = Validation::new(alg);
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes()).into_static();
        Auth {
            iss,
            header,
            validation,
            access_token_lifetime,
            refresh_token_lifetime,
            encoding_key,
            decoding_key,
        }
    }

    pub async fn create_token_pair(&self, id: i32) -> Result<AuthTokens> {
        let time = SystemTime::now();
        let refresh_claims = self.refresh_claims(id.clone(), time);
        let refresh = encode(&self.header, &refresh_claims, &self.encoding_key)?;
        let access_claims = self.access_claims(id.clone(), time);
        let access = encode(&self.header, &access_claims, &self.encoding_key)?;
        Ok(AuthTokens { refresh, access })
    }

    fn refresh_claims(&self, id: i32, time: SystemTime) -> Claims {
        let iat = unix_timestamp(time);
        let exp = unix_timestamp(time + self.refresh_token_lifetime);
        let iss = self.iss.clone();
        let token_type = TokenType::Refresh;
        let sub = id;
        Claims {
            exp,
            iat,
            iss,
            token_type,
            sub,
        }
    }

    fn access_claims(&self, id: i32, time: SystemTime) -> Claims {
        let iat = unix_timestamp(time);
        let exp = unix_timestamp(time + self.access_token_lifetime);
        let token_type = TokenType::Access;
        let iss = self.iss.clone();
        let sub = id.clone();
        Claims {
            exp,
            iat,
            iss,
            token_type,
            sub,
        }
    }

    pub fn decode(&self, token: &str) -> Result<TokenData<Claims>> {
        decode::<Claims>(token, &self.decoding_key, &self.validation).map_err(Into::into)
    }
}

pub fn unix_timestamp(time: SystemTime) -> u64 {
    match time.duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    Access,
    Refresh,
}

// The claims are the contents of the JWT, a base-64 encoded JSON object placed
// as the second part of the JWT, e.g. if the JWT is "xxxxx.yyyyyy.zzzzzz",
// then the claims would be "yyyyyy".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    // Expiration time as a unix timestamp.
    pub exp: u64,
    // Issued time as a unix timestamp.
    pub iat: u64,
    // Issuer name, configured from the outside to always be the same.
    pub iss: String,
    // Token type, should be 'access' or 'refresh' depending on the type.
    pub token_type: TokenType,
    // Subject of the token--whom token refers to. The user id in our case.
    pub sub: i32,
}
