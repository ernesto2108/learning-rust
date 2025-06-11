use axum::{Json};

pub async fn finder_products() -> Json<String>{
    Json("Products found".to_string())
}