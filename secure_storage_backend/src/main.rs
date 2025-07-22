use axum::{Router};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio_cron_scheduler::{Job, JobScheduler};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::filter::LevelFilter;


use crate::{config::Config, db::{DBClient, UserExt}};

mod config;
mod models;
mod dto;
mod error;
mod db;
mod utils;
mod middleware;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .with_test_writer()
        .init();
    dotenv().ok();

    let env = Config::new();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&env.database_url)
        .await {
            Ok(pool) => {
                println!("Connected to database");
                pool
            },
            Err(e) => {
                eprintln!("Error connecting to database: {}", e);
                std::process::exit(1);
            }
        };

    let db_client = DBClient::new(pool);

    let app_state = AppState { 
        env: env.clone(), 
        db_client: db_client.clone() 
    };
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any)
        .allow_credentials(true);

    let scheduler = JobScheduler::new().await.unwrap();
    let job: Job = Job::new_async("0 0 * * * *", move |_, _| {
        let db_client = app_state.db_client.clone();
        Box::pin(async move {
            println!("Running scheduled task to delete expired files");
            match db_client.delete_expired_files().await {
                Ok(_) => println!("Expired files deleted successfully"),
                Err(e) => eprintln!("Error deleting expired files: {}", e),
            }
        })
    }).unwrap();

    scheduler.add(job).await.unwrap();

    tokio::spawn(async move {
        scheduler.start().await.unwrap();
    });

    let app = Router::new().layer(cors.clone());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &env.port)).await.unwrap();
    println!("Server is running on port {}", &env.port);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}