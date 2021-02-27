use crate::common::auth::{
    AccessToken as AuthAccessToken, AuthTokens, RefreshToken as AuthRefreshToken,
};
use crate::store::Store;
use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct RefreshToken {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub token: AuthRefreshToken,
    pub valid: bool,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct AccessToken {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub token: AuthAccessToken,
    pub refresh_token_id: i32,
}

impl Store {
    pub async fn auth_create_tokens(&self, user_id: i32, auth: &AuthTokens) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let refresh = sqlx::query_as::<_, RefreshToken>(
            r#"
INSERT INTO refresh_tokens ( token, valid, user_id )
VALUES ( $1, $2 )
RETURNING id
        "#,
        )
        .bind(&auth.refresh)
        .bind(&true)
        .bind(user_id)
        .fetch_one(&mut tx)
        .await?;

        sqlx::query(
            r#"
INSERT INTO access_tokens ( token, refresh_token_id )
VALUES ( $1, $2 )
RETURNING id
        "#,
        )
        .bind(&auth.refresh)
        .bind(&refresh.id)
        .execute(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn auth_logout(&self, auth: AuthTokens) -> Result<()> {
        sqlx::query(
            r#"
UPDATE refresh_tokens
SET valid=$1
WHERE token=$2
        "#,
        )
        .bind(&auth.refresh)
        .bind(false)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn auth_rotate_tokens(
        &self,
        user_id: i32,
        old: &AuthTokens,
        new: &AuthTokens,
    ) -> Result<AuthTokens> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            r#"
UPDATE refresh_tokens
SET valid=$1
WHERE token=$2
        "#,
        )
        .bind(&old.refresh)
        .bind(false)
        .execute(&mut tx)
        .await?;

        let refresh = sqlx::query_as::<_, RefreshToken>(
            r#"
INSERT INTO refresh_tokens ( token, valid, user_id )
VALUES ( $1, $2 )
RETURNING id
        "#,
        )
        .bind(&new.refresh)
        .bind(&true)
        .bind(user_id)
        .fetch_one(&mut tx)
        .await?;

        let access = sqlx::query_as::<_, AccessToken>(
            r#"
INSERT INTO access_tokens ( token, refresh_token_id )
VALUES ( $1, $2 )
RETURNING id
        "#,
        )
        .bind(&new.access)
        .bind(&refresh.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(AuthTokens {
            refresh: refresh.token,
            access: access.token,
        })
    }
}
