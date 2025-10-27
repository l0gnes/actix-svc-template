use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;

#[cfg(feature = "docs")]
pub mod docs;

#[cfg(feature = "tracing")]
pub mod tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "env")]
    dotenvy::dotenv()?;

    #[cfg(feature = "tracing")]
    tracing::setup_tracing().await?;

    // Setup and run the web server
    HttpServer::new(|| {
        let app = App::new().wrap(TracingLogger::default());

        // Spins up an swagger-ui instance at */swagger-ui/index.html
        #[cfg(feature = "docs")]
        let app = app.service(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui/{_:.*}").url(
            "/api-docs/openapi.json",
            <docs::ApiDoc as utoipa::OpenApi>::openapi(),
        ));

        return app;
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    return Ok(());
}
