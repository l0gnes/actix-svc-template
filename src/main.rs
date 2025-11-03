use actix_web::{App, HttpServer};

#[cfg(feature = "tracing")]
pub mod tracing;

pub mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "env")]
    let _ = dotenvy::dotenv();

    #[cfg(feature = "tracing")]
    tracing::setup_tracing().await?;

    // Setup and run the web server
    HttpServer::new(|| {
        let app = App::new().wrap(tracing::TracingLogger::default());

        let app = app.configure(routes::configure);

        app
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    return Ok(());
}
