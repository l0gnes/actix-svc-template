use actix_web::web::ServiceConfig;

mod example;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(example::get_ping_endpoint);

    #[cfg(feature = "docs")]
    cfg.service(example::docs_example::search_employee_endpoint);
}
