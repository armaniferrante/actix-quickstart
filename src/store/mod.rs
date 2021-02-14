use deadpool_postgres::Pool;

pub(crate) mod person;

#[derive(Clone)]
pub(crate) struct Store {
    pool: Pool,
}

impl Store {
    pub(crate) fn new(pool: Pool) -> Self {
        Self { pool }
    }
}
