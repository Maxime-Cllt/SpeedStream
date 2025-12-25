use crate::core::dto::speed_data::SpeedData;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use tokio::sync::broadcast;

#[derive(Clone)]
#[non_exhaustive]
pub struct AppState {
    pub db: PgPool,
    pub redis: ConnectionManager,
    pub broadcast_tx: broadcast::Sender<SpeedData>,
}

impl AppState {
    /// Creates a new instance of `AppState` with the provided database connection pool, Redis client, and broadcast channel.
    #[inline]
    #[must_use]
    pub fn new(
        db: PgPool,
        redis: ConnectionManager,
        broadcast_tx: broadcast::Sender<SpeedData>,
    ) -> Self {
        Self {
            db,
            redis,
            broadcast_tx,
        }
    }
}
