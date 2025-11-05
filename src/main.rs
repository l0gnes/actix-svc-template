use std::{env, sync::Arc};

use actix_web::{App, HttpServer, web};
use welds::connections::postgres::PostgresClient;

#[cfg(feature = "tracing")]
pub mod tracing;

pub mod db;
pub mod routes;

// The struct which stores all the data that the
// service might use in many different endpoints
pub struct ServiceStore {
    db_client: Arc<PostgresClient>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "env")]
    let _ = dotenvy::dotenv();

    #[cfg(feature = "tracing")]
    tracing::setup_tracing().await?;

    let db_info = env::var("POSTGRES_URL").expect("Failed to get database connection uri");

    let database_client = db::get_postgres_connection(db_info).await;

    // runs database migrations
    db::setup_database(&database_client)
        .await
        .expect("Failed to setup database");

    // Setup and run the web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ServiceStore {
                db_client: Arc::new(database_client.clone()),
            }))
            .wrap(tracing::TracingLogger::default())
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    return Ok(());
}
