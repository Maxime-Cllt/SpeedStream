use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSensorDataRequest {
    pub speed: f32, 
}
