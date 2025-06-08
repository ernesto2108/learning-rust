use rusqlite::{Connection, Result, params}; 
use std::error::Error; 


refinery::embed_migrations!("migrations");


struct Product {
    id: i32,
    name: String,
    price: f64,
    quantity: i32,
}

fn main() -> Result<(), Box<dyn Error>>{
   let mut conn = Connection::open("my_database.db")?; // Needs to be 'mut' because refinery requires a mutable connection
    println!("Database connection established successfully.");

    // Apply database migrations
    migrations::runner().run(&mut conn)?; // Apply the embedded migrations
    println!("Migrations successfully applied.");

    // If everything went well, return Ok(()) to indicate success

    let product_name = "Sample Product";
    let product_price = 19.99;
    let quantity = 100;


    match conn.execute(
        "INSERT INTO products (name, price, quantity) VALUES (?1, ?2, ?3)", 
        params![
            product_name, 
            product_price,
            quantity
        ],
    ) {
        Ok(rows_affected) => println!("Insertado {} fila(s) para el producto: {}", rows_affected, product_name),
        Err(e) => eprintln!("Fallo al insertar el producto: {}", e),
    }

    // Example of querying the database
    let mut stmt = conn.prepare("SELECT id, name, price FROM products")?;
    let product_iter = stmt.query_map([], |row| {
        Ok(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            price: row.get(2)?,
            quantity: row.get(3)?,
        })
    })?;

    println!("Products in the database:");
    for product in product_iter {
        match product {
            Ok(p) => println!("ID: {}, Name: {}, Price: {}, Quantity: {}", p.id, p.name, p.price, p.quantity),
            Err(e) => eprintln!("Error retrieving product: {}", e),
        }
    }
    println!("Database operations completed successfully.");

    Ok(())
}