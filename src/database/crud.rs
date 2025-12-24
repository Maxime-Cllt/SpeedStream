use crate::api::payload::create_speed_request::CreateSpeedDataRequest;
use crate::core::dto::speed_data::SpeedData;
use sqlx::PgPool;
use crate::log_error;
use std::time::Duration;

const QUERY_TIMEOUT: Duration = Duration::from_secs(5);

/// Wraps a database query with a timeout to prevent indefinite hangs
async fn with_timeout<T, F>(fut: F) -> Result<T, sqlx::Error>
where
    F: std::future::Future<Output = Result<T, sqlx::Error>>,
{
    tokio::time::timeout(QUERY_TIMEOUT, fut)
        .await
        .map_err(|_| sqlx::Error::PoolTimedOut)?
}

/// Inserts speed data into the database and returns the inserted record.
#[inline]
pub async fn insert_speed_data(
    db: &PgPool,
    payload: CreateSpeedDataRequest,
) -> Result<SpeedData, sqlx::Error> {
    const QUERY_INSERT: &str =
        "INSERT INTO speed (sensor_name,speed,lane) VALUES (NULLIF($1, ''), $2, $3) RETURNING id, sensor_name, speed, lane, created_at";

    sqlx::query_as::<_, SpeedData>(QUERY_INSERT)
        .bind(payload.sensor_name.unwrap_or_default())
        .bind(payload.speed)
        .bind(i32::from(payload.lane))
        .fetch_one(db)
        .await
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
    let query = sqlx::query_as::<_, SpeedData>(QUERY)
        .bind(i64::from(offset))
        .bind(i64::from(limit))
        .fetch_all(db);

    let rows: Vec<SpeedData> = with_timeout(query)
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
    let query = sqlx::query_as::<_, SpeedData>(QUERY)
        .bind(i64::from(limit))
        .fetch_all(db);

    let rows: Vec<SpeedData> = with_timeout(query)
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
    const QUERY: &str = "SELECT id,sensor_name,speed,lane,created_at FROM speed ORDER BY id DESC LIMIT 1";
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

/// Fetches all speed data entries within a specified date range
#[inline]
pub async fn fetch_speed_data_by_date_range(
    db: &PgPool,
    start_date: chrono::DateTime<chrono::Utc>,
    end_date: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<SpeedData>, sqlx::Error> {
    const QUERY: &str =
        "SELECT id,sensor_name,speed,lane,created_at FROM speed WHERE created_at >= $1 AND created_at <= $2 ORDER BY created_at ASC";

    let query = sqlx::query_as::<_, SpeedData>(QUERY)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(db);

    let rows: Vec<SpeedData> = with_timeout(query)
        .await
        .map_err(|e| {
            log_error!("Failed to fetch speed data by date range: {e}");
            e
        })?;

    Ok(rows
        .into_iter()
        .map(|row| SpeedData::new(row.id, row.sensor_name, row.speed, row.lane, row.created_at))
        .collect())
}
