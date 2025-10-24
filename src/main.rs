use axum::{
    routing::{get, post},
    Router,
};
use speed_stream::constant::DATABASE_URL;
use speed_stream::api::handler::{
    create_speed, get_last_n_speed, get_last_speed, get_speed_pagination, get_speed_today,
    health_check, root,
};
use speed_stream::core::app_state::AppState;
use speed_stream::tracing::logger::Logger;
use speed_stream::tracing::log_level::LogLevel;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Sensor API Server...");

    // Create a logger that writes to "app.log" with minimum level of Info
    Logger::init("app.log", LogLevel::Trace)?;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .connect(DATABASE_URL)
        .await?;

    println!("Connected to Postgres database");

    let app_state = AppState::new(pool);

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/create-speed", post(create_speed))
        .route("/api/get-speed", get(get_last_n_speed))
        .route("/api/get-speed/pagination", get(get_speed_pagination))
        .route("/api/get-speed/today", get(get_speed_today))
        .route("/api/get-speed/last", get(get_last_speed))
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
