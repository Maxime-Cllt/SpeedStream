use sqlx::PgPool;

#[derive(Clone)]
#[non_exhaustive]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {

    /// Creates a new instance of `AppState` with the provided database connection pool.
    #[inline]
    #[must_use]
    pub const fn new(db: PgPool) -> Self {
        Self { db }
    }
}
