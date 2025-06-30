use crate::database::*;
use crate::structs::api_response::ApiResponse;
use crate::structs::data_sensor_request::CreateSensorDataRequest;
use crate::structs::query_limit::QueryLimit;
use crate::{database, AppState};
use axum::extract::Query;
use axum::{extract::State, http::StatusCode, response::Json};

/// Handler functions for the API
pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => Ok(Json(ApiResponse::success(
            "API is healthy",
            "Database connected".into(),
        ))),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// Handler to create speed data from Arduino
pub async fn create_speed_data(
    State(state): State<AppState>,
    Json(payload): Json<CreateSensorDataRequest>,
) -> Result<StatusCode, StatusCode> {
    match insert_speed_data(&state.db, payload).await {
        Ok(bool) => Ok(if bool {
            StatusCode::CREATED
        } else {
            StatusCode::BAD_REQUEST
        }),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Retrieves the last n speed data entries from the database
pub async fn get_speed_n_data(
    State(state): State<AppState>,
    Query(params): Query<QueryLimit>,
) -> Result<Json<Vec<SensorData>>, StatusCode> {
    let limit: u16 = params.limit.unwrap_or(100).min(1000);

    match fetch_last_n_speed_data(&state.db, limit).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            eprintln!("Error fetching speed data: {e:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Root handler for the API
pub async fn root() -> &'static str {
    "Sensor Data API - Running with Axum & MySQL"
}
