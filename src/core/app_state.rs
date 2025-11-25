use sqlx::PgPool;
use redis::aio::ConnectionManager;

#[derive(Clone)]
#[non_exhaustive]
pub struct AppState {
    pub db: PgPool,
    pub redis: ConnectionManager,
}

impl AppState {

    /// Creates a new instance of `AppState` with the provided database connection pool and Redis client.
    #[inline]
    #[must_use]
    pub const fn new(db: PgPool, redis: ConnectionManager) -> Self {
        Self { db, redis }
    }
}
