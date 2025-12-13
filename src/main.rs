use axum::{
    routing::{get, post},
    Router,
};
use redis::Client;
use speed_stream::api::handler::{
    create_speed, get_last_n_speed, get_last_speed, get_speed_by_date_range, get_speed_pagination,
    get_speed_today, health_check, root, speed_stream,
};
use speed_stream::config::constant::{DATABASE_URL, REDIS_URL};
use speed_stream::core::app_state::AppState;
use speed_stream::telemetry::tracing::log_level::LogLevel;
use speed_stream::telemetry::tracing::logger::Logger;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load environment variables from .env file if present
    dotenvy::dotenv().ok();

    println!("Starting Sensor API Server...");

    // Create a logger that writes to "app.log" with minimum level of Info
    Logger::init("app.log", LogLevel::Trace)?;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .connect(DATABASE_URL.as_str())
        .await?;

    println!("Connected to Postgres database");

    // Initialize Redis connection
    let redis_client = Client::open(REDIS_URL.as_str())?;
    let redis_manager = redis::aio::ConnectionManager::new(redis_client).await?;

    println!("Connected to Redis cache");

    // Create broadcast channel for real-time speed notifications
    // Channel capacity of 100 means it can hold up to 100 messages before dropping oldest
    let (broadcast_tx, _) = tokio::sync::broadcast::channel(100);

    let app_state = AppState::new(pool, redis_manager, broadcast_tx);

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        // RESTful endpoints for speed measurements
        .route("/api/speeds", post(create_speed))
        .route("/api/speeds", get(get_last_n_speed))
        .route("/api/speeds/latest", get(get_last_speed))
        .route("/api/speeds/today", get(get_speed_today))
        .route("/api/speeds/paginated", get(get_speed_pagination))
        .route("/api/speeds/range", get(get_speed_by_date_range))
        // Real-time SSE endpoint for speed notifications
        .route("/api/speeds/stream", get(speed_stream))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // Bind to address and serve the application
    let listener: TcpListener = TcpListener::bind("0.0.0.0:8080").await?;

    println!("Listening on http://{}", listener.local_addr()?);

    axum::serve(listener, app).await.map_err(|e| {
        eprintln!("Server error: {e}");
        e
    })?;

    Ok(())
}
