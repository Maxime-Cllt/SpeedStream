use sqlx::PgPool;

#[derive(Clone)]
#[non_exhaustive]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    #[inline]
    pub const fn new(db: PgPool) -> Self {
        AppState { db }
    }
}
