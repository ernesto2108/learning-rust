use rusqlite::{Connection, Result}; 
use std::error::Error; 

refinery::embed_migrations!("migrations");

pub fn sql_client(db_path: &str) -> Result<Connection,Box<dyn Error>> {
    // Open a connection to the SQLite database
    let mut conn = Connection::open(db_path)?;
    println!("Database connection established successfully.");

    migrations::runner().run(&mut conn)?; // Apply the embedded migrations
    println!("Migrations successfully applied.");

    // If everything went well, return Ok(()) to indicate success
    Ok(conn)
}