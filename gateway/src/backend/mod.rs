use crate::common::auth::{unix_timestamp, Auth, AuthTokens};
use crate::common::logging::{self, Logger};
use crate::error::Error;
use crate::gateway::http::Config;
use crate::store::user::{CreateUser, UpdateUser, User};
use crate::store::Store;
use anyhow::Result;
use std::time::SystemTime;

#[derive(Clone)]
pub(crate) struct Backend {
    pub cfg: Config,
    auth: Auth,
    store: Store,
    logger: Logger,
}

// Ctor.
impl Backend {
    pub fn new(cfg: Config, auth: Auth, store: Store) -> Self {
        Self {
            auth,
            store,
            cfg,
            logger: logging::get_logger("backend"),
        }
    }
}

// User.
impl Backend {
    pub async fn create_user(&self, user: &CreateUser) -> Result<User> {
        self.store.create_user(user).await
    }

    pub async fn read_user(&self, id: i32) -> Result<User> {
        self.store.read_user(id).await
    }

    pub async fn all_users(&self) -> Result<Vec<User>> {
        self.store.all_users().await
    }

    pub async fn update_user(&self, user_id: i32, user: &UpdateUser) -> Result<User> {
        if user.id != user_id {
            return Err(Error::Unauthorized.into());
        }
        self.store.update_user(user).await
    }
}

// Auth.
impl Backend {
    pub async fn login(&self, username: &str, password: &str) -> Result<AuthTokens> {
        let user = self.store.read_username(username).await?;
        let matches = argon2::verify_encoded(&user.password, password.as_bytes()).unwrap();
        if !matches {
            return Err(Error::InvalidPassword.into());
        }
        let auth_tokens = self.auth.create_token_pair(user.id).await?;
        self.store.auth_create_tokens(user.id, &auth_tokens).await?;

        Ok(auth_tokens)
    }

    pub async fn logout(&self, auth: AuthTokens) -> Result<()> {
        self.store.auth_logout(auth).await
    }

    pub async fn refresh(&self, user_id: i32, old: AuthTokens) -> Result<AuthTokens> {
        // Check the token has not yet expired.
        let token = self.auth.decode(&old.refresh)?;
        if token.claims.exp < unix_timestamp(SystemTime::now()) {
            return Err(Error::ExpiredRefreshToken.into());
        }
        // Create a new token pair.
        let new = self.auth.create_token_pair(user_id).await?;
        // Save it to the database.
        self.store.auth_rotate_tokens(user_id, &old, &new).await?;

        Ok(new)
    }
}
