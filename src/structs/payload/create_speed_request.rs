use serde::{Deserialize, Serialize};

/// Represents a request to create speed data
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSpeedDataRequest {
    pub speed: f32,     // Speed in km/h
    pub timestamp: u64, // Unix timestamp in milliseconds
}
