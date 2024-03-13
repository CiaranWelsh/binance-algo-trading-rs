use tokio_postgres::{NoTls, Error};

async fn create_database_if_not_exists() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=myuser password=passwd dbname=postgres",
        NoTls,
    )
        .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Check if the database exists
    let rows = client
        .query(
            "SELECT 1 FROM pg_database WHERE datname = $1",
            &[&"binance_assets_db"],
        )
        .await?;

    // If the database does not exist, create it
    if rows.is_empty() {
        client
            .batch_execute(&format!(
                "CREATE DATABASE {}",
                "binance_assets_db"
            ))
            .await?;
        println!("Database created.");
    } else {
        println!("Database already exists.");
    }

    Ok(())
}


async fn initialize_tables() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=myuser password=your_password dbname=binance_assets_db",
        NoTls,
    )
        .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Create the asset_balances table if it doesn't exist
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS asset_balances (
            id SERIAL PRIMARY KEY,
            asset VARCHAR(10) NOT NULL,
            free NUMERIC(18, 8) NOT NULL,
            locked NUMERIC(18, 8) NOT NULL,
            last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
    ").await?;

    println!("Tables initialized.");

    Ok(())
}



// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     create_database_if_not_exists().await?;
//     initialize_tables().await?;
//
//     // Proceed with the rest of your application logic
//
//     Ok(())
// }
