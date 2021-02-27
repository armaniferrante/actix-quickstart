use crate::common::auth::Auth;
use crate::common::auth::AuthTokens;
use crate::common::logging::{self, Logger};
use crate::error::Error;
use crate::store::user::{CreateUser, UpdateUser, User};
use crate::store::Store;
use anyhow::Result;

#[derive(Clone)]
pub(crate) struct Backend {
    auth: Auth,
    store: Store,
    logger: Logger,
}

// Ctor.
impl Backend {
    pub fn new(auth: Auth, store: Store) -> Self {
        Self {
            auth,
            store,
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

    pub async fn update_user(&self, user: &UpdateUser) -> Result<User> {
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

    pub async fn refresh(&self, old: AuthTokens) -> Result<AuthTokens> {
        let user_id = 0; // todo

        let new = self.auth.create_token_pair(user_id).await?;
        self.store.auth_rotate_tokens(user_id, &old, &new).await?;

        Ok(new)
    }
}
