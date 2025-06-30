use crate::structs::data_sensor_request::CreateSensorDataRequest;
pub use crate::structs::sensor_data::SensorData;
use sqlx::MySqlPool;

/// Inserts speed data into the database.
pub async fn insert_speed_data(
    db: &MySqlPool,
    payload: CreateSensorDataRequest,
) -> Result<bool, sqlx::Error> {
    const QUERY_INSERT: &str = "INSERT INTO speed (speed) VALUES (?)";
    sqlx::query(QUERY_INSERT)
        .bind(payload.speed)
        .execute(db)
        .await
        .map(|_| true)
}

/// Fetches the last n speed data entries from the database.
pub async fn fetch_last_n_speed_data(
    db: &MySqlPool,
    number: u16,
) -> Result<Vec<SensorData>, sqlx::Error> {
    const QUERY: &str = "SELECT id, speed, created_at FROM speed ORDER BY id DESC LIMIT ?";
    let rows = sqlx::query_as::<_, SensorData>(QUERY)
        .bind(number as i64)
        .fetch_all(db)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| SensorData::new(row.id, row.speed, row.created_at))
        .collect())
}
