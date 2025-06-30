use crate::structs::data_sensor_request::CreateSensorDataRequest;
use crate::structs::sensor_data::SensorData;
use anyhow::Result;
use sqlx::MySqlPool;

pub async fn insert_speed_data(
    db: &MySqlPool,
    payload: CreateSensorDataRequest,
) -> Result<SensorData> {
    let sensor_data = "INSERT INTO speed (speed) VALUES (?)";
    let sensor_data = sqlx::query_as::<_, SensorData>(sensor_data)
        .bind(payload.speed)
        .fetch_one(db)
        .await?;

    Ok(sensor_data)
}
