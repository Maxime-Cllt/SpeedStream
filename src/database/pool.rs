use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use std::time::Duration;

pub type DbPool = bb8::Pool<PostgresConnectionManager<NoTls>>;

/// Creates an optimized bb8 connection pool for PostgreSQL
///
/// Configuration is optimized for high-throughput, low-latency CRUD operations:
/// - max_size: 20 (down from 30 with SQLx - bb8 is more efficient)
/// - min_idle: 5 (keep warm connections ready)
/// - max_lifetime: 1800s (30 minutes)
/// - idle_timeout: 300s (5 minutes - faster cleanup than SQLx's 600s)
/// - connection_timeout: 3s (down from 5s - fail-fast approach)
/// - test_on_check_out: false (skip validation, Redis handles caching)
pub async fn create_pool(database_url: &str) -> Result<DbPool, Box<dyn std::error::Error>> {
    let config = database_url.parse::<tokio_postgres::Config>()?;
    let manager = PostgresConnectionManager::new(config, NoTls);

    let pool = bb8::Pool::builder()
        .max_size(20)
        .min_idle(Some(5))
        .max_lifetime(Some(Duration::from_secs(1800)))
        .idle_timeout(Some(Duration::from_secs(300)))
        .connection_timeout(Duration::from_secs(3))
        .test_on_check_out(false)
        .build(manager)
        .await?;

    Ok(pool)
}
