use axum::{Router, routing::get};
use crate::product::handler::finder_products;

pub fn routes() -> Router {
    // Define the routes for the application
    let app_routes = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/products", get(finder_products));

    // Return the configured router
    app_routes
}