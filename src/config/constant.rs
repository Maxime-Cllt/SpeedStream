use std::sync::LazyLock;

/// Database connection URL
pub static DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://speedstream:speedstream123@localhost:5432/speedstream_db".to_string()
    })
});

/// Redis connection URL
pub static REDIS_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string())
});
