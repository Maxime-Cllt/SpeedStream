use crate::structs::lane::Lane;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
#[non_exhaustive]
pub struct SpeedData {
    pub id: i32,
    pub speed: f32,                // Represents the speed of the vehicle in km/h
    pub lane: Lane,                // Represents the lane of the vehicle (Left or Right)
    pub created_at: DateTime<Utc>, // Timestamp when the speed data was created
}

impl SpeedData {
    /// Creates a new instance of `SpeedData`.
    #[inline]
    #[must_use]
    pub const fn new(id: i32, speed: f32, lane: Lane, created_at: DateTime<Utc>) -> Self {
        Self {
            id,
            speed,
            lane,
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
        const ID: i32 = 1i32;
        const SPEED: f32 = 10.0;
        let created_at = Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0).unwrap();
        let sensor_data = SpeedData::new(ID, SPEED, Lane::Left, created_at);

        assert_eq!(sensor_data.id, ID);
        assert_eq!(sensor_data.speed, SPEED);
        assert_eq!(sensor_data.lane, Lane::Left);
        assert_eq!(sensor_data.created_at, created_at);
    }
}
