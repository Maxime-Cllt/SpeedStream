use crate::api::payload::create_speed_request::CreateSpeedDataRequest;
use crate::core::dto::speed_data::SpeedData;
use crate::database::pool::DbPool;
use crate::database::types::{DbError, FromPostgresRow};
use crate::database::util::{with_timeout, INSERT_TIMEOUT, SIMPLE_SELECT_TIMEOUT, RANGE_QUERY_TIMEOUT};
use crate::log_error;

/// Inserts speed data into the database and returns the inserted record.
pub async fn insert_speed_data(
    pool: &DbPool,
    payload: CreateSpeedDataRequest,
) -> Result<SpeedData, DbError> {
    const QUERY: &str = "INSERT INTO speed (sensor_name,speed,lane) VALUES (NULLIF($1, ''), $2, $3) RETURNING id, sensor_name, speed, lane, created_at";

    let conn = pool.get().await?;

    let query_future = async {
        let stmt = conn.prepare(QUERY).await.map_err(DbError::from)?;
        let row = conn
            .query_one(
                &stmt,
                &[
                    &payload.sensor_name.unwrap_or_default(),
                    &payload.speed,
                    &i32::from(payload.lane),
                ],
            )
            .await
            .map_err(DbError::from)?;

        SpeedData::from_row(&row)
    };

    with_timeout(query_future, INSERT_TIMEOUT)
        .await
        .map_err(|e| {
            log_error!("Failed to insert speed data: {e}");
            e
        })
}

/// Fetches the last n speed data entries from the database.
pub async fn fetch_last_n_speed_data(
    pool: &DbPool,
    number: u16,
) -> Result<Vec<SpeedData>, DbError> {
    const QUERY: &str =
        "SELECT id,sensor_name,speed,lane,created_at FROM speed ORDER BY id DESC LIMIT $1";

    let conn = pool.get().await?;

    let query_future = async {
        let stmt = conn.prepare(QUERY).await.map_err(DbError::from)?;
        let rows = conn.query(&stmt, &[&(i64::from(number))]).await.map_err(DbError::from)?;

        rows.iter()
            .map(SpeedData::from_row)
            .collect::<Result<Vec<_>, _>>()
    };

    with_timeout(query_future, SIMPLE_SELECT_TIMEOUT)
        .await
        .map_err(|e| {
            log_error!("Failed to fetch last n speed data: {e}");
            e
        })
}

/// Fetches the rows with pagination support.
pub async fn fetch_speed_data_with_pagination(
    pool: &DbPool,
    offset: u32,
    limit: u32,
) -> Result<Vec<SpeedData>, DbError> {
    const QUERY: &str =
        "SELECT id,sensor_name,speed,lane,created_at FROM speed OFFSET $1 LIMIT $2";

    let conn = pool.get().await?;

    let query_future = async {
        let stmt = conn.prepare(QUERY).await.map_err(DbError::from)?;
        let rows = conn
            .query(&stmt, &[&(i64::from(offset)), &(i64::from(limit))])
            .await
            .map_err(DbError::from)?;

        rows.iter()
            .map(SpeedData::from_row)
            .collect::<Result<Vec<_>, _>>()
    };

    with_timeout(query_future, SIMPLE_SELECT_TIMEOUT)
        .await
        .map_err(|e| {
            log_error!("Failed to fetch speed data with pagination: {e}");
            e
        })
}

/// Fetches all rows from inserted in the current day
pub async fn fetch_speed_data_today(
    pool: &DbPool,
    limit: u16,
) -> Result<Vec<SpeedData>, DbError> {
    const QUERY: &str = "SELECT id,sensor_name,speed,lane,created_at FROM speed WHERE created_at >= CURRENT_DATE LIMIT $1";

    let conn = pool.get().await?;

    let query_future = async {
        let stmt = conn.prepare(QUERY).await.map_err(DbError::from)?;
        let rows = conn.query(&stmt, &[&(i64::from(limit))]).await.map_err(DbError::from)?;

        rows.iter()
            .map(SpeedData::from_row)
            .collect::<Result<Vec<_>, _>>()
    };

    with_timeout(query_future, SIMPLE_SELECT_TIMEOUT)
        .await
        .map_err(|e| {
            log_error!("Failed to fetch today's speed data: {e}");
            e
        })
}

/// Fetches the last speed data entry from the database
pub async fn fetch_last_speed(pool: &DbPool) -> Result<SpeedData, DbError> {
    const QUERY: &str =
        "SELECT id,sensor_name,speed,lane,created_at FROM speed ORDER BY id DESC LIMIT 1";

    let conn = pool.get().await?;

    let query_future = async {
        let stmt = conn.prepare(QUERY).await.map_err(DbError::from)?;
        let row = conn.query_one(&stmt, &[]).await.map_err(DbError::from)?;

        SpeedData::from_row(&row)
    };

    with_timeout(query_future, SIMPLE_SELECT_TIMEOUT)
        .await
        .map_err(|e| {
            log_error!("Failed to fetch last speed data: {e}");
            e
        })
}

/// Fetches all speed data entries within a specified date range
pub async fn fetch_speed_data_by_date_range(
    pool: &DbPool,
    start_date: chrono::DateTime<chrono::Utc>,
    end_date: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<SpeedData>, DbError> {
    const QUERY: &str = "SELECT id,sensor_name,speed,lane,created_at FROM speed WHERE created_at >= $1 AND created_at <= $2 ORDER BY created_at ASC";

    let conn = pool.get().await?;

    let query_future = async {
        let stmt = conn.prepare(QUERY).await.map_err(DbError::from)?;
        let rows = conn.query(&stmt, &[&start_date, &end_date]).await.map_err(DbError::from)?;

        rows.iter()
            .map(SpeedData::from_row)
            .collect::<Result<Vec<_>, _>>()
    };

    with_timeout(query_future, RANGE_QUERY_TIMEOUT)
        .await
        .map_err(|e| {
            log_error!("Failed to fetch speed data by date range: {e}");
            e
        })
}
