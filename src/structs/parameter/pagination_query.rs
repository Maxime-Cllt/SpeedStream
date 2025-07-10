use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<u32>,
    pub offest: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pagination_query_serialization() {
        let query = PaginationQuery {
            limit: Some(10),
            offest: Some(20),
        };

        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(serialized, r#"{"limit":10,"offest":20}"#);

        let deserialized: PaginationQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.limit, Some(10));
        assert_eq!(deserialized.offest, Some(20));
    }

    #[tokio::test]
    async fn test_pagination_query_serialization_without_fields() {
        let query = PaginationQuery {
            limit: None,
            offest: None,
        };

        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(serialized, r#"{"limit":null,"offest":null}"#);

        let deserialized: PaginationQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.limit, None);
        assert_eq!(deserialized.offest, None);
    }

    #[tokio::test]
    async fn test_pagination_query_serialization_with_only_limit() {
        let query = PaginationQuery {
            limit: Some(10),
            offest: None,
        };

        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(serialized, r#"{"limit":10,"offest":null}"#);

        let deserialized: PaginationQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.limit, Some(10));
        assert_eq!(deserialized.offest, None);
    }

    #[tokio::test]
    async fn test_pagination_query_serialization_with_only_offset() {
        let query = PaginationQuery {
            limit: None,
            offest: Some(20),
        };

        let serialized = serde_json::to_string(&query).unwrap();
        assert_eq!(serialized, r#"{"limit":null,"offest":20}"#);

        let deserialized: PaginationQuery = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.limit, None);
        assert_eq!(deserialized.offest, Some(20));
    }
}
