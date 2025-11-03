use actix_web::get;

#[get("/ping")]
pub async fn get_ping_endpoint() -> String {
    return "Pong!".to_string();
}
