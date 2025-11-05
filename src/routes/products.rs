use std::ops::Deref;

use crate::{ServiceStore, db::models::Product};
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    get, post,
    web::{self, Data, Json, Path, ServiceConfig},
};
use serde::{Deserialize, Serialize};
use welds::exts::VecStateExt;

pub fn setup_product_endpoints(cfg: &mut ServiceConfig) {
    // actix-governor handles ratelimiting using governor
    let governor_config = GovernorConfigBuilder::default()
        .requests_per_minute(10)
        .burst_size(5)
        .finish()
        .unwrap();

    cfg.service(
        web::scope("/products")
            .wrap(Governor::new(&governor_config))
            .service(post_new_product)
            .service(get_all_products)
            .service(get_product_by_id),
    );
}

#[derive(Deserialize, Serialize)]
struct ProductCreationRequest {
    name: String,
    price: f32,
}

#[post("")]
pub async fn post_new_product(
    state: Data<ServiceStore>,
    payload: Json<ProductCreationRequest>,
) -> Json<Product> {
    let mut new_product = Product::new();
    new_product.name = payload.name.clone();
    new_product.price = payload.price.clone();

    new_product
        .save(&(*state.db_client))
        .await
        .expect("Failed to create new product");

    return Json(new_product.into_inner());
}

#[get("")]
pub async fn get_all_products(state: Data<ServiceStore>) -> Json<Vec<Product>> {
    let products = Product::all()
        .run(&(*state.db_client))
        .await
        .expect("Failed to retrieve products");

    return Json(products.into_inners());
}

#[get("/{id}")]
pub async fn get_product_by_id(
    state: Data<ServiceStore>,
    path: Path<(i32,)>,
) -> Json<Option<Product>> {
    let req_id = path.into_inner().0.clone();

    let prod = Product::where_col(|p| p.id.equal(req_id))
        .run(&(*state.db_client))
        .await
        .unwrap();

    let first = prod.first();

    if let Some(f) = first {
        let inner = f.deref().clone();

        return Json(Some(inner));
    } else {
        return Json(None);
    }
}
