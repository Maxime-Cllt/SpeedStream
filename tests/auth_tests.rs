use speed_stream::database::cache::generate_token_cache_key;
use speed_stream::middleware::auth::extract_bearer_token;

/// Integration tests for authentication logic
/// These tests verify the behavior without requiring external connections

#[test]
fn test_token_extraction_flow() {
    // Simulate a complete token extraction flow
    let auth_header = "Bearer mySecretToken123";

    // Extract token
    let token = extract_bearer_token(auth_header);
    assert!(token.is_some());

    let token = token.unwrap();
    assert_eq!(token, "mySecretToken123");

    // Generate cache key for this token
    let cache_key = generate_token_cache_key(token);
    assert_eq!(cache_key, "speedstream:token:mySecretToken123");
}

#[test]
fn test_invalid_token_extraction_flow() {
    // Test with invalid auth headers
    let invalid_headers = vec![
        "Basic mytoken",           // Wrong scheme
        "Bearer",                   // Missing token
        "bearer mytoken",          // Wrong case
        "mytoken",                 // No scheme
        "",                        // Empty
    ];

    for header in invalid_headers {
        let token = extract_bearer_token(header);
        assert!(token.is_none(), "Header '{}' should be invalid", header);
    }
}

#[test]
fn test_cache_key_consistency() {
    // Test that the same token always generates the same cache key
    let token = "consistent_token_123";

    let key1 = generate_token_cache_key(token);
    let key2 = generate_token_cache_key(token);

    assert_eq!(key1, key2);
    assert_eq!(key1, "speedstream:token:consistent_token_123");
}

#[test]
fn test_different_tokens_different_keys() {
    // Test that different tokens generate different cache keys
    let token1 = "token_abc";
    let token2 = "token_xyz";

    let key1 = generate_token_cache_key(token1);
    let key2 = generate_token_cache_key(token2);

    assert_ne!(key1, key2);
}

#[test]
fn test_complete_auth_header_processing() {
    // Test a realistic scenario from start to finish
    let valid_headers = vec![
        ("Bearer abc123", "abc123"),
        ("Bearer xyz-789_ABC.def", "xyz-789_ABC.def"),
        ("Bearer a", "a"),
        ("Bearer 1234567890", "1234567890"),
    ];

    for (header, expected_token) in valid_headers {
        let token = extract_bearer_token(header);
        assert!(token.is_some(), "Failed to extract token from: {}", header);

        let token = token.unwrap();
        assert_eq!(token, expected_token);

        // Verify cache key generation
        let cache_key = generate_token_cache_key(token);
        assert!(cache_key.starts_with("speedstream:token:"));
        assert!(cache_key.ends_with(expected_token));
    }
}

#[test]
fn test_token_length_variations() {
    // Test with tokens of various lengths
    let short_token = "a";
    let medium_token = "abc123def456ghi789";
    let long_token = "a".repeat(100);

    // All should be extractable
    let short_header = format!("Bearer {}", short_token);
    let medium_header = format!("Bearer {}", medium_token);
    let long_header = format!("Bearer {}", &long_token);

    assert_eq!(
        extract_bearer_token(&short_header),
        Some(short_token)
    );
    assert_eq!(
        extract_bearer_token(&medium_header),
        Some(&medium_token as &str)
    );
    assert_eq!(
        extract_bearer_token(&long_header),
        Some(&long_token as &str)
    );

    // All should generate valid cache keys
    assert_eq!(
        generate_token_cache_key(short_token),
        format!("speedstream:token:{}", short_token)
    );
    assert_eq!(
        generate_token_cache_key(&medium_token),
        format!("speedstream:token:{}", medium_token)
    );
    assert_eq!(
        generate_token_cache_key(&long_token),
        format!("speedstream:token:{}", long_token)
    );
}

#[test]
fn test_edge_cases() {
    // Test edge cases that might occur in production

    // Token with only numbers
    let numeric_token = extract_bearer_token("Bearer 1234567890");
    assert_eq!(numeric_token, Some("1234567890"));

    // Token with special characters that are URL-safe
    let special_token = extract_bearer_token("Bearer abc-123_DEF.456~xyz");
    assert_eq!(special_token, Some("abc-123_DEF.456~xyz"));

    // Very long token (like a JWT)
    let jwt_like = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    let jwt_header = format!("Bearer {}", jwt_like);
    let jwt_token = extract_bearer_token(&jwt_header);
    assert_eq!(jwt_token, Some(jwt_like));
}
