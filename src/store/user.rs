use crate::error::Error;
use crate::store::Store;
use anyhow::Result;
use argon2::Config;
use chrono::NaiveDateTime;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub email: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub verified: bool,
}

#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email)]
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub id: i32,
    pub username: String,
}

impl Store {
    pub async fn create_user(&self, user: &CreateUser) -> Result<User> {
        let salt = salt()?;
        let config = Config::default();
        let password_hash = argon2::hash_encoded(user.password.as_bytes(), &salt, &config)?;
        sqlx::query_as::<_, User>(
            r#"
INSERT INTO users ( username, email, password )
VALUES ( $1, $2, $3 )
RETURNING id
        "#,
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(password_hash)
        .fetch_one(&self.conn)
        .await
        .map_err(Into::into)
    }

    pub async fn read_user(&self, id: i32) -> Result<User> {
        sqlx::query_as::<_, User>(&format!("SELECT * from users where id={0}", id))
            .fetch_one(&self.conn)
            .await
            .map_err(Into::into)
    }

    pub async fn read_username(&self, username: &str) -> Result<User> {
        sqlx::query_as::<_, User>(&format!("SELECT * from users where username={0}", username))
            .fetch_one(&self.conn)
            .await
            .map_err(Into::into)
    }

    pub async fn all_users(&self) -> Result<Vec<User>> {
        sqlx::query_as::<_, User>(&format!("SELECT * from users"))
            .fetch_all(&self.conn)
            .await
            .map_err(Into::into)
    }

    pub async fn update_user(&self, user: &UpdateUser) -> Result<User> {
        sqlx::query_as::<_, User>(
            r#"
UPDATE users
SET username=$2
WHERE users.id=$3
"#,
        )
        .bind(&user.username)
        .bind(&user.id)
        .fetch_one(&self.conn)
        .await
        .map_err(Into::into)
    }
}

fn salt() -> Result<Vec<u8>, Error> {
    let len = 8;
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; len as usize];
    rng.try_fill_bytes(bytes.as_mut_slice())?;
    Ok(bytes)
}
