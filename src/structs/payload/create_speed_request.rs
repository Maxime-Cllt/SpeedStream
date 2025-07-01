use serde::{Deserialize, Serialize};

/// Represents a request to create speed data
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSpeedDataRequest {
    pub speed: f32,
}
