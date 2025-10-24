use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryLimit {
    pub limit: Option<u16>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_limit_deserialization() {
        let json_data = r#"{"limit": 10}"#;
        let query_limit: QueryLimit = serde_json::from_str(json_data).unwrap();
        assert_eq!(query_limit.limit, Some(10));

        let json_data_no_limit = r#"{}"#;
        let query_limit_no_limit: QueryLimit = serde_json::from_str(json_data_no_limit).unwrap();
        assert_eq!(query_limit_no_limit.limit, None);
    }

    #[tokio::test]
    async fn test_query_limit_deserialization_invalid() {
        let json_data_invalid = r#"{"limit": "invalid"}"#;
        let result: Result<QueryLimit, _> = serde_json::from_str(json_data_invalid);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_query_limit_deserialization_edge_cases() {
        let json_data_empty = r#"{}"#;
        let query_limit_empty: QueryLimit = serde_json::from_str(json_data_empty).unwrap();
        assert_eq!(query_limit_empty.limit, None);

        let json_data_null = r#"{"limit": null}"#;
        let query_limit_null: QueryLimit = serde_json::from_str(json_data_null).unwrap();
        assert_eq!(query_limit_null.limit, None);
    }

    #[tokio::test]
    async fn test_query_limit_deserialization_large_value() {
        let json_data_large = r#"{"limit": 10000}"#;
        let query_limit_large: QueryLimit = serde_json::from_str(json_data_large).unwrap();
        assert_eq!(query_limit_large.limit, Some(10000));
    }

    #[tokio::test]
    async fn test_query_limit_deserialization_zero() {
        let json_data_zero = r#"{"limit": 0}"#;
        let query_limit_zero: QueryLimit = serde_json::from_str(json_data_zero).unwrap();
        assert_eq!(query_limit_zero.limit, Some(0));
    }

    #[tokio::test]
    async fn test_query_limit_deserialization_negative() {
        let json_data_negative = r#"{"limit": -5}"#;
        let result: Result<QueryLimit, _> = serde_json::from_str(json_data_negative);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_query_limit_deserialization_non_integer() {
        let json_data_float = r#"{"limit": 10.5}"#;
        let result: Result<QueryLimit, _> = serde_json::from_str(json_data_float);
        assert!(result.is_err());
    }
}
