use std::future::Future;
use std::time::Duration;
use crate::database::types::DbError;

/// Optimized timeout for INSERT operations
///
/// INSERT operations with proper indexes
/// should complete quickly. Longer timeout may indicate database issues.
pub const INSERT_TIMEOUT: Duration = Duration::from_secs(3);

/// Optimized timeout for simple SELECT queries
///
/// Simple SELECTs with LIMIT and indexes
/// complete in <100ms typically. 2s is generous for network latency.
pub const SIMPLE_SELECT_TIMEOUT: Duration = Duration::from_secs(2);

/// Timeout for range queries (date ranges, pagination)
///
/// Range queries may scan more rows,
/// but with proper indexes should still complete quickly.
pub const RANGE_QUERY_TIMEOUT: Duration = Duration::from_secs(4);

/// Optimized timeout for authentication queries
///
/// Auth queries are critical path and
/// Redis-cached, so should be very fast. Low timeout acceptable.
pub const AUTH_QUERY_TIMEOUT: Duration = Duration::from_secs(1);

/// Wraps a database operation with a timeout
pub async fn with_timeout<T, F>(fut: F, timeout: Duration) -> Result<T, DbError>
where
    F: Future<Output = Result<T, DbError>>,
{
    tokio::time::timeout(timeout, fut)
        .await
        .map_err(|_| DbError::Timeout)?
}
