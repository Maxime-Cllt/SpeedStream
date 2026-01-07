use crate::core::lane::Lane;
use crate::database::types::FromPostgresRow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents speed data collected from a sensor on track
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
#[must_use]
pub struct SpeedData {
    pub id: i32,
    pub sensor_name: Option<String>, // Optional name of the sensor
    pub speed: f32,                  // Represents the speed of the vehicle in km/h
    pub lane: Lane,                  // Represents the lane of the vehicle (Left or Right)
    pub created_at: DateTime<Utc>,   // Timestamp when the speed data was created
}

impl SpeedData {
    /// Creates a new instance of `SpeedData`.
    #[inline]
    pub fn new(
        id: i32,
        sensor_name: Option<String>,
        speed: f32,
        lane: Lane,
        created_at: DateTime<Utc>,
    ) -> Self {
        SpeedData {
            id,
            sensor_name,
            speed,
            lane,
            created_at,
        }
    }
}

impl FromPostgresRow for SpeedData {
    fn from_row(row: &tokio_postgres::Row) -> Result<Self, crate::database::types::DbError> {
        use crate::database::types::DbError;

        Ok(SpeedData {
            id: row.try_get("id").map_err(DbError::from)?,
            sensor_name: row.try_get("sensor_name").map_err(DbError::from)?,
            speed: row.try_get("speed").map_err(DbError::from)?,
            lane: Lane::try_from(row.try_get::<_, i32>("lane").map_err(DbError::from)?)
                .map_err(|e| DbError::RowParsing(format!("Invalid lane value: {}", e)))?,
            created_at: row.try_get("created_at").map_err(DbError::from)?,
        })
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
        let sensor_data = SpeedData::new(
            ID,
            Some("Sensor A".to_string()),
            SPEED,
            Lane::Left,
            created_at,
        );

        assert_eq!(sensor_data.id, ID);
        assert_eq!(sensor_data.sensor_name.as_deref(), Some("Sensor A"));
        assert_eq!(sensor_data.speed, SPEED);
        assert_eq!(sensor_data.lane, Lane::Left);
        assert_eq!(sensor_data.created_at, created_at);
    }
}
