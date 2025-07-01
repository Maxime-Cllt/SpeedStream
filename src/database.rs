use crate::structs::payload::create_speed_request::CreateSpeedDataRequest;
pub use crate::structs::sensor_data::SensorData;
use sqlx::PgPool;

/// Inserts speed data into the database.
pub async fn insert_speed_data(
    db: &PgPool,
    payload: CreateSpeedDataRequest,
) -> Result<bool, sqlx::Error> {
    const QUERY_INSERT: &str = "INSERT INTO speed (speed) VALUES ($1)";
    sqlx::query(QUERY_INSERT)
        .bind(payload.speed)
        .execute(db)
        .await
        .map(|_| true)
}

/// Fetches the last n speed data entries from the database.
pub async fn fetch_last_n_speed_data(
    db: &PgPool,
    number: u16,
) -> Result<Vec<SensorData>, sqlx::Error> {
    const QUERY: &str = "SELECT id, speed, created_at FROM speed ORDER BY id DESC LIMIT $1";
    let rows: Vec<SensorData> = sqlx::query_as::<_, SensorData>(QUERY)
        .bind(number as i64)
        .fetch_all(db)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| SensorData::new(row.id, row.speed, row.created_at))
        .collect())
}

/// Fetches the rows with pagination support.
pub async fn fetch_speed_data_with_pagination(
    db: &PgPool,
    offset: u32,
    limit: u32,
) -> Result<Vec<SensorData>, sqlx::Error> {
    const QUERY: &str = "SELECT id, speed, created_at FROM speed OFFSET $1 LIMIT $2";
    let rows: Vec<SensorData> = sqlx::query_as::<_, SensorData>(QUERY)
        .bind(i64::from(offset))
        .bind(i64::from(limit))
        .fetch_all(db)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| SensorData::new(row.id, row.speed, row.created_at))
        .collect())
}

/// Fetches all rows from inserted in the current day
pub async fn fetch_speed_data_today(
    db: &PgPool,
    limit: u16,
) -> Result<Vec<SensorData>, sqlx::Error> {
    const QUERY: &str =
        "SELECT id, speed, created_at FROM speed WHERE created_at >= CURRENT_DATE LIMIT $1";
    let rows: Vec<SensorData> = sqlx::query_as::<_, SensorData>(QUERY)
        .bind(i64::from(limit))
        .fetch_all(db)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| SensorData::new(row.id, row.speed, row.created_at))
        .collect())
}
