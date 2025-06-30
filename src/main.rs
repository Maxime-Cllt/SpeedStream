use crate::handler::{create_speed_data, health_check};
use crate::structs::app_state::AppState;
use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use sqlx::mysql::MySqlPoolOptions;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};

mod database;
mod handler;
mod structs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const DATABASE_URL: &str = "mysql://HHBL8703:@localhost:3308/test";

    tracing_subscriber::fmt()
        .with_env_filter("sensor_api=debug,tower_http=debug")
        .init();

    info!("Starting Sensor API Server...");

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&DATABASE_URL)
        .await?;

    info!("Connected to MySQL database");

    let app_state = AppState { db: pool };

    // Build the application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/create-speed", post(create_speed_data))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await?;

    info!("Listening on http://{}", listener.local_addr()?);

    axum::serve(listener, app).await.map_err(|e| {
        error!("Server error: {e}");
        e
    })?;

    Ok(())
}

async fn root() -> &'static str {
    "Sensor Data API - Running with Axum & MySQL"
}
