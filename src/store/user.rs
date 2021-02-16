use crate::store::Store;
use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(email)]
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl Store {
    pub async fn create_user(&self, user: &CreateUser) -> Result<User> {
        sqlx::query_as::<_, User>(
            r#"
INSERT INTO users ( first_name, last_name, email )
VALUES ( $1, $2, $3 )
RETURNING id
        "#,
        )
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }

    pub async fn read_user(&self, id: i32) -> Result<User> {
        sqlx::query_as::<_, User>(&format!("SELECT * from users where id={0}", id))
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }

    pub async fn all_users(&self) -> Result<Vec<User>> {
        sqlx::query_as::<_, User>(&format!("SELECT * from users"))
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }

    pub async fn update_user(&self, user: &User) -> Result<User> {
        sqlx::query_as::<_, User>(
            r#"
UPDATE users
SET email=$1, first_name=$2, last_name=$3
WHERE users.id=$4
"#,
        )
        .bind(&user.email)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.id)
        .fetch_one(&self.pool)
        .await
        .map_err(Into::into)
    }
}
