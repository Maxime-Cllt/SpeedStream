use crate::log_error;
use sqlx::{PgPool, Row};

/// Validates if a token exists and is active in the database
#[inline]
pub async fn validate_token(db: &PgPool, token: &str) -> Result<bool, sqlx::Error> {
    
    // Query the database for the token
    let result = sqlx::query(
        r#"
        SELECT is_active
        FROM api_keys
        WHERE api_key = $1
        "#,
    )
    .bind(token)
    .fetch_optional(db)
    .await
    .map_err(|e| {
        log_error!("Failed to validate token in database: {e}");
        e
    })?;

    // Return true only if token exists and is active
    match result {
        Some(row) => Ok(row.try_get::<bool, _>("is_active").unwrap_or(false)),
        None => Ok(false),
    }
}
