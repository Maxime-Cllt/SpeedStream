use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SensorData {
    pub id: i32,
    pub speed: f32,
    pub created_at: DateTime<Utc>,
}

impl SensorData {
    #[inline]
    pub const fn new(id: i32, speed: f32, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            speed,
            created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone as _;

    #[tokio::test]
    async fn test_sensor_data_creation() {
        let id = 1i32;
        let speed: f32 = 10.0;
        let created_at = Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0).unwrap();
        let sensor_data = SensorData::new(id, speed, created_at);

        assert_eq!(sensor_data.id, id);
        assert_eq!(sensor_data.speed, speed);
        assert_eq!(sensor_data.created_at, created_at);
    }
}
