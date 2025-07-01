use crate::database::*;
use crate::structs::api_response::ApiResponse;
use crate::structs::app_state::AppState;
use crate::structs::data_sensor_request::CreateSensorDataRequest;
use crate::structs::pagination_query::PaginationQuery;
use crate::structs::query_limit::QueryLimit;
use axum::extract::Query;
use axum::{extract::State, http::StatusCode, response::Json};

/// Handler functions for the API
pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => {
            let today: String = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            Ok(Json(ApiResponse::success(
                "API is healthy",
                format!("{today} - Database connection is active"),
            )))
        }
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// Handler to create speed data from Arduino
pub async fn create_speed(
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
pub async fn get_last_n_speed(
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

/// Retrieves speed data with pagination support
pub async fn get_speed_pagination(
    State(state): State<AppState>,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<Vec<SensorData>>, StatusCode> {
    let offset: u32 = params.offest.unwrap_or(0);
    let limit: u32 = params.limit.unwrap_or(100).min(1000);

    match fetch_speed_data_with_pagination(&state.db, offset, limit).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            eprintln!("Error fetching speed data with pagination: {e:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Root handler for the API
pub async fn root() -> &'static str {
    "Sensor Data API - Running with Axum & Postgres"
}
