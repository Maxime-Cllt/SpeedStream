use crate::handler::{create_speed_data, get_speed_n_data, health_check, root};
use crate::structs::app_state::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::mysql::MySqlPoolOptions;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod database;
mod handler;
mod structs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const DATABASE_URL: &str = "mysql://HHBL8703:@localhost:3308/test";

    println!("Starting Sensor API Server...");

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(DATABASE_URL)
        .await?;

    println!("Connected to MySQL database");

    let app_state = AppState { db: pool };

    // Build the application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/create-speed", post(create_speed_data))
        .route("/api/get-speed", get(get_speed_n_data))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("Listening on http://{}", listener.local_addr()?);

    axum::serve(listener, app).await.map_err(|e| {
        eprintln!("Server error: {e}");
        e
    })?;

    Ok(())
}
