use crate::log_error;
use crate::structs::payload::create_speed_request::CreateSpeedDataRequest;
use crate::structs::speed_data::SpeedData;
use sqlx::PgPool;

/// Inserts speed data into the database.
#[inline]
pub async fn insert_speed_data(
    db: &PgPool,
    payload: CreateSpeedDataRequest,
) -> Result<bool, sqlx::Error> {
    const QUERY_INSERT: &str =
        "INSERT INTO speed (sensor_name,speed,lane) VALUES (NULLIF($1, ''), $2, $3)";

    sqlx::query(QUERY_INSERT)
        .bind(payload.sensor_name.unwrap_or_default())
        .bind(payload.speed)
        .bind(i32::from(payload.lane))
        .execute(db)
        .await
        .map(|_| true)
        .map_err(|e| {
            log_error!("Failed to insert speed data: {e}");
            e
        })
}

/// Fetches the last n speed data entries from the database.
#[inline]
pub async fn fetch_last_n_speed_data(
    db: &PgPool,
    number: u16,
) -> Result<Vec<SpeedData>, sqlx::Error> {
    const QUERY: &str =
        "SELECT id,sensor_name,speed,lane,created_at FROM speed ORDER BY id DESC LIMIT $1";
    let rows: Vec<SpeedData> = sqlx::query_as::<_, SpeedData>(QUERY)
        .bind(i64::from(number))
        .fetch_all(db)
        .await
        .map_err(|e| {
            log_error!("Failed to fetch last n speed data: {e}");
            e
        })?;
    Ok(rows
        .into_iter()
        .map(|row| SpeedData::new(row.id, row.sensor_name, row.speed, row.lane, row.created_at))
        .collect())
}

/// Fetches the rows with pagination support.
#[inline]
pub async fn fetch_speed_data_with_pagination(
    db: &PgPool,
    offset: u32,
    limit: u32,
) -> Result<Vec<SpeedData>, sqlx::Error> {
    const QUERY: &str = "SELECT id,sensor_name,speed,lane,created_at FROM speed OFFSET $1 LIMIT $2";
    let rows: Vec<SpeedData> = sqlx::query_as::<_, SpeedData>(QUERY)
        .bind(i64::from(offset))
        .bind(i64::from(limit))
        .fetch_all(db)
        .await
        .map_err(|e| {
            log_error!("Failed to fetch speed data with pagination: {e}");
            e
        })?;
    Ok(rows
        .into_iter()
        .map(|row| SpeedData::new(row.id, row.sensor_name, row.speed, row.lane, row.created_at))
        .collect())
}

/// Fetches all rows from inserted in the current day
#[inline]
pub async fn fetch_speed_data_today(
    db: &PgPool,
    limit: u16,
) -> Result<Vec<SpeedData>, sqlx::Error> {
    const QUERY: &str = "SELECT id,sensor_name,speed,lane,created_at FROM speed WHERE created_at >= CURRENT_DATE LIMIT $1";
    let rows: Vec<SpeedData> = sqlx::query_as::<_, SpeedData>(QUERY)
        .bind(i64::from(limit))
        .fetch_all(db)
        .await
        .map_err(|e| {
            log_error!("Failed to fetch today's speed data: {e}");
            e
        })?;

    Ok(rows
        .into_iter()
        .map(|row| SpeedData::new(row.id, row.sensor_name, row.speed, row.lane, row.created_at))
        .collect())
}

#[inline]
pub async fn fetch_last_speed(db: &PgPool) -> Result<SpeedData, sqlx::Error> {
    const QUERY: &str = "SELECT id,sensor_name,speed,lane,created_at FROM speed ORDER BY id DESC";
    let row: SpeedData = sqlx::query_as::<_, SpeedData>(QUERY)
        .fetch_one(db)
        .await
        .map_err(|e| {
            log_error!("Failed to fetch last speed data: {e}");
            e
        })?;
    Ok(SpeedData::new(
        row.id,
        row.sensor_name,
        row.speed,
        row.lane,
        row.created_at,
    ))
}
