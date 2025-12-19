use crate::api::payload::create_speed_request::CreateSpeedDataRequest;
use crate::api::query::date_range_query::DateRangeQuery;
use crate::api::query::pagination_query::PaginationQuery;
use crate::api::query::query_limit::QueryLimit;
use crate::core::app_state::AppState;
use crate::core::dto::speed_data::SpeedData;
use crate::database::cache::*;
use crate::database::crud::*;
use crate::log_error;
use axum::extract::Query;
use axum::{extract::State, http::StatusCode, response::Json};
use axum::response::sse::{Event, Sse};
use futures_util::stream::Stream;
use std::convert::Infallible;

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

/// Handler to create speed data from Arduino (with cache update and real-time broadcast)
#[inline]
pub async fn create_speed(
    State(mut state): State<AppState>,
    Json(payload): Json<CreateSpeedDataRequest>,
) -> Result<StatusCode, StatusCode> {
    match insert_speed_data(&state.db, payload).await {
        Ok(speed_data) => {
            // Update cache with the newly inserted data
            if let Err(e) = set_last_speed_in_cache(&mut state.redis, &speed_data).await {
                log_error!("Failed to update cache after insert: {e:?}");
            }

            // Broadcast the new speed data to all connected clients
            // We ignore the result because it's OK if no one is listening
            let _ = state.broadcast_tx.send(speed_data);

            Ok(StatusCode::CREATED)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Retrieves the last n speed data entries from the database
#[inline]
pub async fn get_last_n_speed(
    State(state): State<AppState>,
    Query(params): Query<QueryLimit>,
) -> Result<Json<Vec<SpeedData>>, StatusCode> {
    let limit: u16 = params.limit.unwrap_or(100).min(1000);

    match fetch_last_n_speed_data(&state.db, limit).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            log_error!("Error fetching speed data: {e:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Retrieves speed data with pagination support
#[inline]
pub async fn get_speed_pagination(
    State(state): State<AppState>,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<Vec<SpeedData>>, StatusCode> {
    let offset: u32 = params.offest.unwrap_or(0);
    let limit: u32 = params.limit.unwrap_or(100).min(1000);

    match fetch_speed_data_with_pagination(&state.db, offset, limit).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            log_error!("Error fetching speed data with pagination: {e:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Retrieves all speed data entries inserted today
#[inline]
pub async fn get_speed_today(
    State(state): State<AppState>,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<Vec<SpeedData>>, StatusCode> {
    let limit: u16 = u16::try_from(params.limit.unwrap_or(100).min(1000)).unwrap_or(100);
    match fetch_speed_data_today(&state.db, limit).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            log_error!("Error fetching today's speed data: {e:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Retrieves the last speed data entry (with Redis caching)
#[inline]
pub async fn get_last_speed(
    State(mut state): State<AppState>,
) -> Result<Json<SpeedData>, StatusCode> {
    // Try to get from cache first
    match get_last_speed_from_cache(&mut state.redis).await {
        Ok(Some(cached_data)) => {
            return Ok(Json(cached_data));
        }
        _ => {
            // Cache miss or error, proceed to fetch from database
        }
    }

    // If not in cache or cache error, fetch from database
    match fetch_last_speed(&state.db).await {
        Ok(data) => {
            // Update cache asynchronously (best effort - don't fail if cache update fails)
            if let Err(e) = set_last_speed_in_cache(&mut state.redis, &data).await {
                log_error!("Failed to update cache: {e:?}");
            }
            Ok(Json(data))
        }
        Err(e) => {
            log_error!("Error fetching last speed data: {e:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Root handler for the API
#[inline]
pub async fn root() -> &'static str {
    "Sensor Data API - Running with Axum & Postgres - v1.1.0"
}

/// Retrieves all speed data entries within a specified date range
#[inline]
pub async fn get_speed_by_date_range(
    State(state): State<AppState>,
    Query(params): Query<DateRangeQuery>,
) -> Result<Json<Vec<SpeedData>>, StatusCode> {
    // Parse the start and end dates
    let start_date = match params.parse_start_date() {
        Ok(date) => date,
        Err(e) => {
            log_error!("Invalid start_date format: {e:?}");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let end_date = match params.parse_end_date() {
        Ok(date) => date,
        Err(e) => {
            log_error!("Invalid end_date format: {e:?}");
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Fetch data from database
    match fetch_speed_data_by_date_range(&state.db, start_date, end_date).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            log_error!("Error fetching speed data by date range: {e:?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Server-Sent Events endpoint for real-time speed notifications
/// Clients can connect to this endpoint to receive speed updates as they happen
#[inline]
pub async fn speed_stream(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // Subscribe to the broadcast channel
    let mut rx = state.broadcast_tx.subscribe();

    // Create a stream that yields SSE events
    let stream = async_stream::stream! {
        while let Ok(speed_data) = rx.recv().await {
            // Serialize the speed data to JSON
            if let Ok(json) = serde_json::to_string(&speed_data) {
                // Yield an SSE event with the JSON data
                yield Ok(Event::default().data(json));
            }
        }
    };

    Sse::new(stream)
}
