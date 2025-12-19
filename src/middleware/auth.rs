use crate::core::app_state::AppState;
use crate::database::auth::validate_token;
use crate::database::cache::{cache_valid_token, is_token_cached};
use crate::log_error;
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

/// Extracts Bearer token from Authorization header value
///
/// Returns Some(token) if the header is valid, None otherwise
pub fn extract_bearer_token(auth_header: &str) -> Option<&str> {
    if !auth_header.starts_with("Bearer ") {
        return None;
    }

    let token = &auth_header[7..]; // Skip "Bearer "

    if token.is_empty() {
        return None;
    }

    Some(token)
}

/// Middleware to validate Bearer token authentication
///
/// This middleware:
/// 1. Extracts the Bearer token from the Authorization header
/// 2. Checks if the token is cached in Redis (fast path)
/// 3. If not cached, validates against the database
/// 4. If valid, caches the token for future requests
/// 5. Returns 401 Unauthorized if token is missing or invalid
pub async fn auth_middleware(
    State(mut state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            log_error!("Missing Authorization header");
            StatusCode::UNAUTHORIZED
        })?;

    // Extract the Bearer token
    let token = extract_bearer_token(auth_header).ok_or_else(|| {
        log_error!("Invalid Authorization header format - must be 'Bearer <token>'");
        StatusCode::UNAUTHORIZED
    })?;

    // First, check if token is cached (fast path)
    match is_token_cached(&mut state.redis, token).await {
        Ok(true) => {
            // Token is cached and valid, proceed with request
            return Ok(next.run(request).await);
        }
        Ok(false) => {
            // Token not in cache, need to validate against database
        }
        Err(e) => {
            log_error!("Redis error while checking token cache: {e}");
            // Continue to database validation even if Redis fails
        }
    }

    // Validate token against database
    match validate_token(&state.db, token).await {
        Ok(true) => {
            // Token is valid, cache it for future requests
            if let Err(e) = cache_valid_token(&mut state.redis, token).await {
                log_error!("Failed to cache valid token: {e}");
                // Continue anyway - this is just an optimization
            }

            // Proceed with the request
            Ok(next.run(request).await)
        }
        Ok(false) => {
            log_error!("Invalid or inactive token");
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            log_error!("Database error while validating token: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bearer_token_valid() {
        let token = extract_bearer_token("Bearer abc123def456");
        assert_eq!(token, Some("abc123def456"));
    }

    #[test]
    fn test_extract_bearer_token_with_special_chars() {
        let token = extract_bearer_token("Bearer abc-123_DEF.456~xyz");
        assert_eq!(token, Some("abc-123_DEF.456~xyz"));
    }

    #[test]
    fn test_extract_bearer_token_missing_bearer_prefix() {
        let token = extract_bearer_token("abc123def456");
        assert_eq!(token, None);
    }

    #[test]
    fn test_extract_bearer_token_wrong_prefix() {
        let token = extract_bearer_token("Basic abc123def456");
        assert_eq!(token, None);
    }

    #[test]
    fn test_extract_bearer_token_empty_token() {
        let token = extract_bearer_token("Bearer ");
        assert_eq!(token, None);
    }

    #[test]
    fn test_extract_bearer_token_case_sensitive() {
        // Should fail because "bearer" is lowercase
        let token = extract_bearer_token("bearer abc123def456");
        assert_eq!(token, None);
    }

    #[test]
    fn test_extract_bearer_token_with_extra_spaces() {
        // Should extract everything after "Bearer "
        let token = extract_bearer_token("Bearer  abc123  def456  ");
        assert_eq!(token, Some(" abc123  def456  "));
    }

    #[test]
    fn test_extract_bearer_token_long_token() {
        let long_token = "a".repeat(256);
        let auth_header = format!("Bearer {}", long_token);
        let token = extract_bearer_token(&auth_header);
        assert_eq!(token, Some(long_token.as_str()));
    }

    #[test]
    fn test_extract_bearer_token_empty_string() {
        let token = extract_bearer_token("");
        assert_eq!(token, None);
    }
}
