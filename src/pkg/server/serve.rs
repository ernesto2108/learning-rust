use axum;
use std::error::Error;

use super::routes;

pub async fn run() -> Result<(), Box<dyn Error>>{
    let app = routes::routes();

    let listen = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on localhost:3000");
    axum::serve(
        listen,
        app,
    )
    .await?;

    Ok(())
}