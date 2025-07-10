use serde::{Deserialize, Serialize};

/// Represents a request to create speed data
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSpeedDataRequest {
    pub speed: f32,     // Speed in km/h
    pub lane: u8, // Lane represented as an unsigned 8-bit integer, see `Lane` enum for details
    pub timestamp: u64, // Unix timestamp in milliseconds
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
   async  fn test_create_speed_data_request_serialization() {
        let request = CreateSpeedDataRequest {
            speed: 60.0,
            lane: 2,
            timestamp: 1633072800000,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert_eq!(
            serialized,
            r#"{"speed":60.0,"lane":2,"timestamp":1633072800000}"#
        );

        let deserialized: CreateSpeedDataRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.speed, 60.0);
        assert_eq!(deserialized.lane, 2);
        assert_eq!(deserialized.timestamp, 1633072800000);
    }
}
