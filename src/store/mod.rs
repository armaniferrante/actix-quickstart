use sqlx::postgres::PgPool;

pub(crate) mod user;

#[derive(Clone)]
pub(crate) struct Store {
    pool: PgPool,
}

impl Store {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
