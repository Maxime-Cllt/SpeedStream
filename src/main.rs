use axum::{
    routing::{get, post},
    Router,
};
use speed_stream::constant::DATABASE_URL;
use speed_stream::handler::{
    create_speed, get_last_n_speed, get_speed_pagination, get_speed_today, health_check, root,
};
use speed_stream::structs::app_state::AppState;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Sensor API Server...");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(DATABASE_URL)
        .await?;

    println!("Connected to Postgres database");

    let app_state = AppState { db: pool };

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/create-speed", post(create_speed))
        .route("/api/get-speed", get(get_last_n_speed))
        .route("/api/get-speed/pagination", get(get_speed_pagination))
        .route("/api/get-speed/today", get(get_speed_today))
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
