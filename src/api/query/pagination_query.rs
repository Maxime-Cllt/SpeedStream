use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<u32>,
    /// Offset for pagination (correct spelling)
    /// Accepts both "offset" and "offest" (legacy typo) for backward compatibility
    #[serde(alias = "offest")]
    pub offset: Option<u32>,
}

impl PaginationQuery {
    /// Gets the offset value for pagination
    pub fn get_offset(&self) -> Option<u32> {
        self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pagination_query_serialization() {
        let query = PaginationQuery {
            limit: Some(10),
            offset: Some(20),
        };

        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(serialized, r#"{"limit":10,"offset":20}"#);

        let deserialized: PaginationQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.limit, Some(10));
        assert_eq!(deserialized.offset, Some(20));
    }

    #[tokio::test]
    async fn test_pagination_query_serialization_without_fields() {
        let query = PaginationQuery {
            limit: None,
            offset: None,
        };

        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(serialized, r#"{"limit":null,"offset":null}"#);

        let deserialized: PaginationQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.limit, None);
        assert_eq!(deserialized.offset, None);
    }

    #[tokio::test]
    async fn test_pagination_query_serialization_with_only_limit() {
        let query = PaginationQuery {
            limit: Some(10),
            offset: None,
        };

        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(serialized, r#"{"limit":10,"offset":null}"#);

        let deserialized: PaginationQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.limit, Some(10));
        assert_eq!(deserialized.offset, None);
    }

    #[tokio::test]
    async fn test_pagination_query_serialization_with_only_offset() {
        let query = PaginationQuery {
            limit: None,
            offset: Some(20),
        };

        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(serialized, r#"{"limit":null,"offset":20}"#);

        let deserialized: PaginationQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.limit, None);
        assert_eq!(deserialized.offset, Some(20));
    }

    #[tokio::test]
    async fn test_pagination_query_backward_compatibility() {
        // Test that old API clients using "offest" (typo) still work via serde alias
        let json_with_typo = r#"{"limit":10,"offest":20}"#;
        let deserialized: PaginationQuery = serde_json::from_str(json_with_typo).unwrap();
        assert_eq!(deserialized.get_offset(), Some(20));
        assert_eq!(deserialized.limit, Some(10));

        // Test that new clients using "offset" (correct spelling) work
        let json_correct = r#"{"limit":10,"offset":30}"#;
        let deserialized: PaginationQuery = serde_json::from_str(json_correct).unwrap();
        assert_eq!(deserialized.get_offset(), Some(30));
        assert_eq!(deserialized.limit, Some(10));
    }

    #[tokio::test]
    async fn test_pagination_query_get_offset_method() {
        let query = PaginationQuery {
            limit: Some(10),
            offset: Some(42),
        };
        assert_eq!(query.get_offset(), Some(42));

        let query_none = PaginationQuery {
            limit: Some(10),
            offset: None,
        };
        assert_eq!(query_none.get_offset(), None);
    }
}
