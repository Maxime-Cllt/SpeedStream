use std::sync::LazyLock;

/// Database connection URL
pub static DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    let user = std::env::var("POSTGRES_USER").unwrap_or_else(|_| "speedstream_user".to_string());
    let password = std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "speedstream123".to_string());
    let host = std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "postgres".to_string());
    let port = std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
    let db = std::env::var("POSTGRES_DB").unwrap_or_else(|_| "speedstream_db".to_string());

    format!("postgresql://{user}:{password}@{host}:{port}/{db}")
});

/// Redis connection URL
pub static REDIS_URL: LazyLock<String> = LazyLock::new(|| {
    let host = std::env::var("REDIS_HOST").unwrap_or_else(|_| "redis".to_string());
    let port = std::env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    format!("redis://{host}:{port}")
});

/// Allowed CORS origins
pub static ALLOWED_ORIGINS: LazyLock<Vec<String>> = LazyLock::new(|| {
    std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
});
