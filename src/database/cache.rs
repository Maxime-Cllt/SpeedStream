use crate::core::speed_data::SpeedData;
use crate::log_error;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;

const LAST_SPEED_KEY: &str = "speedstream:last_speed";
const CACHE_TTL: i64 = 3600; // 1 hour TTL

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
