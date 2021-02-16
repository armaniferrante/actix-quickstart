use crate::common::logging::{self, Logger};
use crate::store::user::{CreateUser, User};
use crate::store::Store;
use anyhow::Result;

#[derive(Clone)]
pub(crate) struct Backend {
    store: Store,
    logger: Logger,
}

impl Backend {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            logger: logging::get_logger("backend"),
        }
    }

    pub async fn create_user(&self, user: &CreateUser) -> Result<User> {
        self.store.create_user(user).await
    }

    pub async fn read_user(&self, id: i32) -> Result<User> {
        self.store.read_user(id).await
    }

    pub async fn all_users(&self) -> Result<Vec<User>> {
        self.store.all_users().await
    }

    pub async fn update_user(&self, user: &User) -> Result<User> {
        self.store.update_user(user).await
    }
}
