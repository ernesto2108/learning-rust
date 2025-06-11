mod pkg;
mod product;

use std::error::Error;
use crate::pkg::server::serve;
use crate::pkg::database::conn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
     // Initialize the database connection
    conn::sql_client("./my_database.db").await?;
    println!("Database connection established successfully.");

    serve::run().await?;
    println!("Server has started successfully.");
   
    Ok(())
}