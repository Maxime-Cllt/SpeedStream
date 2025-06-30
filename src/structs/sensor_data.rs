use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SensorData {
    pub id: u32,
    pub speed: u16,
    pub created_at: DateTime<Utc>,
}
