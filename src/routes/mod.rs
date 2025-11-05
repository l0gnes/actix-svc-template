use actix_web::web::ServiceConfig;

use crate::routes::products::setup_product_endpoints;

mod example;
mod products;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(example::get_ping_endpoint);

    cfg.configure(setup_product_endpoints);

    #[cfg(feature = "docs")]
    cfg.service(example::docs_example::search_employee_endpoint);
}
