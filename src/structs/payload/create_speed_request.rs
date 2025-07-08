use serde::{Deserialize, Serialize};

/// Represents a request to create speed data
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSpeedDataRequest {
    pub speed: f32,     // Speed in km/h
    pub lane: u8, // Lane represented as an unsigned 8-bit integer, see `Lane` enum for details
    pub timestamp: u64, // Unix timestamp in milliseconds
}
