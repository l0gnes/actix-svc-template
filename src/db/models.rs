use serde::{Deserialize, Serialize};
use welds::WeldsModel;

#[derive(Debug, WeldsModel, Serialize, Deserialize, Clone)]
#[welds(table = "products")]
pub struct Product {
    #[welds(rename = "product_id")]
    #[welds(primary_key)]
    pub id: i32,
    pub name: String,
    pub price: f32,
}
