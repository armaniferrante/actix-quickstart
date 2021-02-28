use sqlx::postgres::PgPool;

pub(crate) mod auth;
pub(crate) mod user;

#[derive(Clone)]
pub(crate) struct Store {
    conn: PgPool,
}

impl Store {
    pub(crate) fn new(conn: PgPool) -> Self {
        Self { conn }
    }
}
