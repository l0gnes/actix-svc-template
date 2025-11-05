use std::{env, fmt::Display};

use tracing::Level;
use welds::connections::postgres::PostgresClient;

mod migrations;
pub mod models;

pub async fn get_postgres_connection<T: Display>(uri: T) -> PostgresClient {
    let db_connection = welds::connections::postgres::connect(&uri.to_string())
        .await
        .unwrap();

    return db_connection;
}

pub async fn setup_database(client: &PostgresClient) -> anyhow::Result<()> {
    let db_setup_span = tracing::span!(Level::INFO, "db_setup").entered();

    tracing::info!("Running pending database migrations...");

    migrations::run_migrations(client).await;

    tracing::info!("Database migrations completed!");

    db_setup_span.exit();

    return Ok(());
}
