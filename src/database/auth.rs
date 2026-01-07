use crate::database::pool::DbPool;
use crate::database::types::DbError;
use crate::database::util::{with_timeout, AUTH_QUERY_TIMEOUT};
use crate::log_error;

/// Validates if a token exists and is active in the database
pub async fn validate_token(pool: &DbPool, token: &str) -> Result<bool, DbError> {
    const QUERY: &str = "SELECT is_active FROM api_keys WHERE api_key = $1";

    let conn = pool.get().await?;

    let query_future = async {
        let stmt = conn.prepare(QUERY).await.map_err(DbError::from)?;
        let rows = conn.query(&stmt, &[&token]).await.map_err(DbError::from)?;

        match rows.first() {
            Some(row) => Ok(row.try_get::<_, bool>("is_active").unwrap_or(false)),
            None => Ok(false),
        }
    };

    with_timeout(query_future, AUTH_QUERY_TIMEOUT)
        .await
        .map_err(|e| {
            log_error!("Failed to validate token in database: {e}");
            e
        })
}
