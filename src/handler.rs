use crate::structs::api_response::ApiResponse;
use crate::structs::data_sensor_request::CreateSensorDataRequest;
use crate::structs::sensor_data::SensorData;
use crate::{database, AppState};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use tracing::{error, info};

/// Handler functions for the API
pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => Ok(Json(ApiResponse::success(
            "API is healthy",
            "Database connected".to_string(),
        ))),
        Err(e) => {
            error!("Database health check failed: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Handler to create speed data from Arduino
pub async fn create_speed_data(
    State(state): State<AppState>,
    Json(payload): Json<CreateSensorDataRequest>,
) -> Result<Json<ApiResponse<SensorData>>, StatusCode> {
    info!("Received sensor data from Arduino");

    match database::insert_speed_data(&state.db, payload).await {
        Ok(sensor_data) => {
            info!(
                "Successfully inserted sensor data with ID: {}",
                sensor_data.id
            );
            Ok(Json(ApiResponse::success(
                "Data inserted successfully",
                sensor_data,
            )))
        }
        Err(e) => {
            error!("Failed to insert sensor data: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
