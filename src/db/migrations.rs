use welds::{
    Client, TransactStart,
    migrations::{self, MigrationStep, TableState, types::Type},
};

pub async fn run_migrations<T: Client + TransactStart>(client: &T) {
    welds::migrations::up(client, &[create_products_table])
        .await
        .expect("Failed to perform database migrations");
}

// An example database migration to setup the products table
fn create_products_table(_: &TableState) -> welds::errors::Result<MigrationStep> {
    let m = migrations::create_table("products")
        .id(|c| c("product_id", Type::Int))
        .column(|c| c("name", Type::String))
        .column(|c| c("price", Type::Float));

    return Ok(MigrationStep::new("create_products_table", m));
}
