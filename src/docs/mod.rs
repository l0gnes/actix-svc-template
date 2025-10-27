use actix_web::web::ServiceConfig;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
pub struct ApiDoc;

