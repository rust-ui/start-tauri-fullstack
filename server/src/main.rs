#![recursion_limit = "256"]

use std::env;

use dotenv::dotenv;
use leptos::prelude::*;
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod app_router;
mod fallback;

use app_router::build_app_router::build_app_router;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    LogTracer::init().expect("Failed to set logger");

    let subscriber = FmtSubscriber::builder()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        // Apply the EnvFilter to use RUST_LOG
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Could not set subscriber");
    dotenv().ok();

    let conf = get_configuration(None)?;

    let addr = conf.leptos_options.site_addr;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool =
        PgPoolOptions::new().max_connections(5).connect(&database_url).await.unwrap_or_else(|_| {
            panic!("Failed to create Postgres connection pool! URL: {database_url}")
        });

    // Run database migrations
    info!("Running database migrations...");
    sqlx::migrate!("../migrations").run(&pool).await?;
    info!("Database migrations completed successfully! ✔️");

    let app = build_app_router(conf, pool).await?;
    info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
