use crate::core::dto::speed_data::SpeedData;
use crate::database::pool::DbPool;
use redis::aio::ConnectionManager;
use tokio::sync::broadcast;

#[derive(Clone)]
#[non_exhaustive]
pub struct AppState {
    pub db: DbPool,
    pub redis: ConnectionManager,
    pub broadcast_tx: broadcast::Sender<SpeedData>,
}

impl AppState {
    /// Creates a new instance of `AppState` with the provided database connection pool, Redis client, and broadcast channel.
    #[inline]
    #[must_use]
    pub fn new(
        db: DbPool,
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
