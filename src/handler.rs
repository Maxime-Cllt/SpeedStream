use crate::database::*;
use crate::structs::api_response::ApiResponse;
use crate::structs::app_state::AppState;
use crate::structs::data_sensor_request::CreateSensorDataRequest;
use crate::structs::query_limit::QueryLimit;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constant::DATABASE_URL;
    use crate::structs::app_state::AppState;
    use sqlx::mysql::MySqlPoolOptions;

    #[tokio::test]
    async fn test_health_check() {
        let state = AppState {
            db: MySqlPoolOptions::new()
                .max_connections(1)
                .connect(DATABASE_URL)
                .await
                .unwrap(),
        };
        let response = health_check(State(state)).await;
        assert!(response.is_ok());
        let json_response = response.unwrap();
        assert!(json_response.0.success);
    }

    #[tokio::test]
    async fn test_create_speed_data() {
        let state = AppState {
            db: MySqlPoolOptions::new()
                .max_connections(1)
                .connect(DATABASE_URL)
                .await
                .unwrap(),
        };
        let payload = CreateSensorDataRequest { speed: Some(42) };
        let response = create_speed_data(State(state), Json(payload)).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_get_speed_n_data() {
        let state = AppState {
            db: MySqlPoolOptions::new()
                .max_connections(1)
                .connect(DATABASE_URL)
                .await
                .unwrap(),
        };
        let params = QueryLimit { limit: Some(10) };
        let response = get_speed_n_data(State(state), Query(params)).await;
        assert!(response.is_ok());
        let data = response.unwrap().0;
        assert!(!data.is_empty());
    }

    #[tokio::test]
    async fn test_root() {
        let response = root().await;
        assert_eq!(response, "Sensor Data API - Running with Axum & MySQL");
    }
}
