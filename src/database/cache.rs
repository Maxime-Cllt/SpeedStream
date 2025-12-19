use crate::core::dto::speed_data::SpeedData;
use crate::log_error;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;

const LAST_SPEED_KEY: &str = "speedstream:last_speed";
const CACHE_TTL: i64 = 3600; // 1 hour TTL
const TOKEN_CACHE_PREFIX: &str = "speedstream:token:";
const TOKEN_CACHE_TTL: u32 = 86400; // 24 hours TTL for valid tokens

/// Generates a cache key for a token
///
/// This function is public for testing purposes
#[inline]
pub fn generate_token_cache_key(token: &str) -> String {
    format!("{TOKEN_CACHE_PREFIX}{token}")
}

/// Retrieves the last speed data from Redis cache
#[inline]
pub async fn get_last_speed_from_cache(
    redis: &mut ConnectionManager,
) -> Result<Option<SpeedData>, redis::RedisError> {
    let cached: Option<String> = redis.get(LAST_SPEED_KEY).await.map_err(|e| {
        log_error!("Failed to get last speed from cache {e}");
        e
    })?;

    match cached {
        Some(json_str) => match serde_json::from_str::<SpeedData>(&json_str) {
            Ok(data) => Ok(Some(data)),
            Err(e) => {
                log_error!("Failed to deserialize speed data from cache {e}");
                Ok(None)
            }
        },
        None => Ok(None),
    }
}

/// Sets the last speed data in Redis cache
#[inline]
pub async fn set_last_speed_in_cache(
    redis: &mut ConnectionManager,
    speed_data: &SpeedData,
) -> Result<(), redis::RedisError> {
    let json_str = serde_json::to_string(speed_data).map_err(|e| {
        log_error!("Failed to serialize speed data for cache{e} ");
        redis::RedisError::from(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Serialization error: {e}"),
        ))
    })?;

    let _: () = redis
        .set_ex(LAST_SPEED_KEY, json_str, CACHE_TTL as u64)
        .await
        .map_err(|e| {
            log_error!("Failed to set last speed in cache {e}");
            e
        })?;

    Ok(())
}

/// Invalidates the last speed cache entry
#[inline]
pub async fn invalidate_last_speed_cache(
    redis: &mut ConnectionManager,
) -> Result<(), redis::RedisError> {
    let _: () = redis.del(LAST_SPEED_KEY).await.map_err(|e| {
        log_error!("Failed to invalidate last speed cache {e}");
        e
    })?;

    Ok(())
}

/// Checks if a token is valid in the cache
#[inline]
pub async fn is_token_cached(
    redis: &mut ConnectionManager,
    token: &str,
) -> Result<bool, redis::RedisError> {
    let key = generate_token_cache_key(token);
    let exists: bool = redis.exists(&key).await.map_err(|e| {
        log_error!("Failed to check token in cache: {e}");
        e
    })?;
    Ok(exists)
}

/// Caches a valid token with TTL
#[inline]
pub async fn cache_valid_token(
    redis: &mut ConnectionManager,
    token: &str,
) -> Result<(), redis::RedisError> {
    let key = generate_token_cache_key(token);
    let _: () = redis
        .set_ex(&key, "1", TOKEN_CACHE_TTL as u64)
        .await
        .map_err(|e| {
            log_error!("Failed to cache valid token: {e}");
            e
        })?;
    Ok(())
}

/// Invalidates a cached token
#[inline]
pub async fn invalidate_token_cache(
    redis: &mut ConnectionManager,
    token: &str,
) -> Result<(), redis::RedisError> {
    let key = generate_token_cache_key(token);
    let _: () = redis.del(&key).await.map_err(|e| {
        log_error!("Failed to invalidate token cache: {e}");
        e
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token_cache_key() {
        let token = "abc123def456";
        let key = generate_token_cache_key(token);
        assert_eq!(key, "speedstream:token:abc123def456");
    }

    #[test]
    fn test_generate_token_cache_key_empty() {
        let token = "";
        let key = generate_token_cache_key(token);
        assert_eq!(key, "speedstream:token:");
    }

    #[test]
    fn test_generate_token_cache_key_special_chars() {
        let token = "abc-123_DEF.456~xyz";
        let key = generate_token_cache_key(token);
        assert_eq!(key, "speedstream:token:abc-123_DEF.456~xyz");
    }

    #[test]
    fn test_generate_token_cache_key_long_token() {
        let token = "a".repeat(256);
        let key = generate_token_cache_key(&token);
        assert_eq!(key, format!("speedstream:token:{}", token));
        assert_eq!(key.len(), "speedstream:token:".len() + 256);
    }

    #[test]
    fn test_generate_token_cache_key_with_spaces() {
        let token = "abc 123 def";
        let key = generate_token_cache_key(token);
        assert_eq!(key, "speedstream:token:abc 123 def");
    }

    #[test]
    fn test_cache_ttl_constants() {
        // Verify TTL constants are reasonable
        assert_eq!(CACHE_TTL, 3600); // 1 hour
        assert_eq!(TOKEN_CACHE_TTL, 86400); // 24 hours
        assert!(TOKEN_CACHE_TTL as i64 > CACHE_TTL); // Token cache should be longer
    }

    #[test]
    fn test_cache_key_prefix() {
        // Verify the prefix is what we expect
        assert_eq!(TOKEN_CACHE_PREFIX, "speedstream:token:");
        assert_eq!(LAST_SPEED_KEY, "speedstream:last_speed");
    }
}
