use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SensorData {
    pub id: u32,
    pub speed: u16,
    pub created_at: DateTime<Utc>,
}

impl SensorData {
    pub const fn new(id: u32, speed: u16, created_at: DateTime<Utc>) -> Self {
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
    use chrono::TimeZone;

    #[tokio::test]
    async fn test_sensor_data_creation() {
        let id = 1;
        let speed = 100;
        let created_at = Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0).unwrap();
        let sensor_data = SensorData::new(id, speed, created_at);

        assert_eq!(sensor_data.id, id);
        assert_eq!(sensor_data.speed, speed);
        assert_eq!(sensor_data.created_at, created_at);
    }
}
