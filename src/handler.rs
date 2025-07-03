use crate::database::{
    fetch_last_n_speed_data, fetch_speed_data_today, fetch_speed_data_with_pagination,
    insert_speed_data,
};
use crate::structs::app_state::AppState;
use crate::structs::parameter::pagination_query::PaginationQuery;
use crate::structs::parameter::query_limit::QueryLimit;
use crate::structs::payload::create_speed_request::CreateSpeedDataRequest;
use crate::structs::sensor_data::SensorData;
use axum::extract::Query;
use axum::{extract::State, http::StatusCode, response::Json};

/// Handler functions for the API
#[inline]
pub async fn health_check(State(state): State<AppState>) -> Result<Json<String>, StatusCode> {
    match sqlx::query("SELECT 1").fetch_one(&state.db).await {
        Ok(_) => {
            let today: String = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            Ok(Json(format!("API is healthy! Current time: {today}")))
        }
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// Handler to create speed data from Arduino
#[inline]
pub async fn create_speed(
    State(state): State<AppState>,
    Json(payload): Json<CreateSpeedDataRequest>,
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
#[inline]
pub async fn get_last_n_speed(
    State(state): State<AppState>,
    Query(params): Query<QueryLimit>,
) -> Result<Json<Vec<SensorData>>, StatusCode> {
    let limit: u16 = params.limit.unwrap_or(100).min(1000);

    match fetch_last_n_speed_data(&state.db, limit).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            println!("Error fetching speed data: {e:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Retrieves speed data with pagination support
#[inline]
pub async fn get_speed_pagination(
    State(state): State<AppState>,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<Vec<SensorData>>, StatusCode> {
    let offset: u32 = params.offest.unwrap_or(0);
    let limit: u32 = params.limit.unwrap_or(100).min(1000);

    match fetch_speed_data_with_pagination(&state.db, offset, limit).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            println!("Error fetching speed data with pagination: {e:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Retrieves all speed data entries inserted today
#[inline]
pub async fn get_speed_today(
    State(state): State<AppState>,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<Vec<SensorData>>, StatusCode> {
    let limit: u16 = u16::try_from(params.limit.unwrap_or(100).min(1000)).unwrap_or(100);
    match fetch_speed_data_today(&state.db, limit).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            println!("Error fetching today's speed data: {e:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Root handler for the API
#[inline]
pub async fn root() -> &'static str {
    "Sensor Data API - Running with Axum & Postgres"
}
