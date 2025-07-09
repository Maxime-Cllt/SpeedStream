use crate::structs::payload::create_speed_request::CreateSpeedDataRequest;
use crate::structs::speed_data::SpeedData;
use chrono::{DateTime, MappedLocalTime, TimeZone, Utc};
use sqlx::PgPool;

/// Inserts speed data into the database.
#[inline]
pub async fn insert_speed_data(
    db: &PgPool,
    payload: CreateSpeedDataRequest,
) -> Result<bool, sqlx::Error> {
    const QUERY_INSERT: &str = "INSERT INTO speed (speed,lane,created_at) VALUES ($1, $2, $3)";

    // Convert the timestamp from milliseconds to a DateTime<Utc>
    let date: DateTime<Utc> = match Utc.timestamp_millis_opt(payload.timestamp as i64) {
        MappedLocalTime::Single(dt) => dt,
        _ => Utc::now(),
    };

    sqlx::query(QUERY_INSERT)
        .bind(payload.speed)
        .bind(i32::from(payload.lane))
        .bind(date)
        .execute(db)
        .await
        .map(|_| true)
}

/// Fetches the last n speed data entries from the database.
#[inline]
pub async fn fetch_last_n_speed_data(
    db: &PgPool,
    number: u16,
) -> Result<Vec<SpeedData>, sqlx::Error> {
    const QUERY: &str = "SELECT id,speed,lane,created_at FROM speed ORDER BY id DESC LIMIT $1";
    let rows: Vec<SpeedData> = sqlx::query_as::<_, SpeedData>(QUERY)
        .bind(i64::from(number))
        .fetch_all(db)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| SpeedData::new(row.id, row.speed, row.lane, row.created_at))
        .collect())
}

/// Fetches the rows with pagination support.
#[inline]
pub async fn fetch_speed_data_with_pagination(
    db: &PgPool,
    offset: u32,
    limit: u32,
) -> Result<Vec<SpeedData>, sqlx::Error> {
    const QUERY: &str = "SELECT id,speed,lane,created_at FROM speed OFFSET $1 LIMIT $2";
    let rows: Vec<SpeedData> = sqlx::query_as::<_, SpeedData>(QUERY)
        .bind(i64::from(offset))
        .bind(i64::from(limit))
        .fetch_all(db)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| SpeedData::new(row.id, row.speed, row.lane, row.created_at))
        .collect())
}

/// Fetches all rows from inserted in the current day
#[inline]
pub async fn fetch_speed_data_today(
    db: &PgPool,
    limit: u16,
) -> Result<Vec<SpeedData>, sqlx::Error> {
    const QUERY: &str =
        "SELECT id,speed,lane,created_at FROM speed WHERE created_at >= CURRENT_DATE LIMIT $1";
    let rows: Vec<SpeedData> = sqlx::query_as::<_, SpeedData>(QUERY)
        .bind(i64::from(limit))
        .fetch_all(db)
        .await?;
    Ok(rows
        .into_iter()
        .map(|row| SpeedData::new(row.id, row.speed, row.lane, row.created_at))
        .collect())
}

#[inline]
pub async fn fetch_last_speed(db: &PgPool) -> Result<SpeedData, sqlx::Error> {
    const QUERY: &str = "SELECT id,speed,lane,created_at FROM speed ORDER BY id DESC";
    let row: SpeedData = sqlx::query_as::<_, SpeedData>(QUERY).fetch_one(db).await?;
    Ok(SpeedData::new(row.id, row.speed, row.lane, row.created_at))
}
