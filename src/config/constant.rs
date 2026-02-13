use std::sync::LazyLock;

/// Database connection URL
/// Priority: POSTGRES_URL > individual POSTGRES_* variables
pub static DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    if let Ok(url) = std::env::var("POSTGRES_URL") {
        if !url.is_empty() {
            return url;
        }
    }

    let user = std::env::var("POSTGRES_USER").unwrap_or_else(|_| "speedstream".to_string());
    let password =
        std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "speedstream123".to_string());
    let host = std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "postgres".to_string());
    let port = std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
    let db = std::env::var("POSTGRES_DB").unwrap_or_else(|_| "speedstream_db".to_string());

    format!("postgresql://{user}:{password}@{host}:{port}/{db}")
});

/// Redis connection URL with authentication
/// Priority: REDIS_URL > individual REDIS_* variables
pub static REDIS_URL: LazyLock<String> = LazyLock::new(|| {
    if let Ok(url) = std::env::var("REDIS_URL") {
        if !url.is_empty() {
            return url;
        }
    }

    let host = std::env::var("REDIS_HOST").unwrap_or_else(|_| "redis".to_string());
    let port = std::env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());

    // Support for Redis password authentication
    match std::env::var("REDIS_PASSWORD") {
        Ok(password) if !password.is_empty() => {
            format!("redis://:{password}@{host}:{port}")
        }
        _ => {
            format!("redis://{host}:{port}")
        }
    }
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

/// Server host
pub static HOST: LazyLock<String> = LazyLock::new(|| {
    std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string())
});

/// Server port
pub static PORT: LazyLock<u16> = LazyLock::new(|| {
    std::env::var("SERVER_PORT")
        .or_else(|_| std::env::var("PORT"))
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("SERVER_PORT must be a number")
});
