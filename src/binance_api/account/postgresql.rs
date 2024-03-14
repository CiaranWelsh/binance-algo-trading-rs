use tokio_postgres::{NoTls, Error};

/*
Certainly! Setting up an OrderJournal involves several steps, including designing the database schema, implementing the logic to insert and update records, and possibly creating a way to analyze the data. Here’s a basic guide to get you started:

1. Database Schema Design
First, design a schema for your database that can capture all the relevant information about each order. Here’s a simplified example for a PostgreSQL database:

sql
Copy code
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    binance_order_id BIGINT UNIQUE NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    order_type VARCHAR(10) NOT NULL,
    side VARCHAR(10) NOT NULL,
    order_status VARCHAR(20) NOT NULL,
    price DECIMAL(20,10) NOT NULL,
    original_quantity DECIMAL(20,10) NOT NULL,
    executed_quantity DECIMAL(20,10) NOT NULL,
    time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    strategy VARCHAR(50),
    notes TEXT
);
2. Inserting Orders into the Journal
Whenever you place an order through the Binance API, insert a record into your database. Here’s an example in Rust, assuming you are using the tokio-postgres crate for asynchronous PostgreSQL communication:

rust
Copy code
use tokio_postgres::{NoTls, Error};

async fn insert_order(
    client: &tokio_postgres::Client,
    binance_order_id: i64,
    symbol: &str,
    order_type: &str,
    side: &str,
    order_status: &str,
    price: f64,
    original_quantity: f64,
    executed_quantity: f64,
    strategy: &str,
    notes: &str,
) -> Result<(), Error> {
    client.execute(
        "INSERT INTO orders (binance_order_id, symbol, order_type, side, order_status, price, original_quantity, executed_quantity, strategy, notes) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        &[&binance_order_id, &symbol, &order_type, &side, &order_status, &price, &original_quantity, &executed_quantity, &strategy, &notes],
    ).await?;
    Ok(())
}
3. Updating Orders
Orders might change status (filled, partially filled, etc.), so you need a way to update them. Here’s a function to update an order's status and executed quantity:

rust
Copy code
async fn update_order_status(
    client: &tokio_postgres::Client,
    binance_order_id: i64,
    new_status: &str,
    executed_quantity: f64,
) -> Result<(), Error> {
    client.execute(
        "UPDATE orders SET order_status = $2, executed_quantity = $3, update_time = CURRENT_TIMESTAMP WHERE binance_order_id = $1",
        &[&binance_order_id, &new_status, &executed_quantity],
    ).await?;
    Ok(())
}
4. Querying the Journal
To analyze your trading activity, you’ll create queries against your orders table. For example, to get the total executed quantity of a particular symbol:

rust
Copy code
async fn total_executed_for_symbol(
    client: &tokio_postgres::Client,
    symbol: &str,
) -> Result<f64, Error> {
    let row = client.query_one(
        "SELECT SUM(executed_quantity) FROM orders WHERE symbol = $1",
        &[&symbol],
    ).await?;

    Ok(row.get::<_, f64>(0))
}
5. Running the Application
To run this application, you'll need to manage database connections, possibly using a connection pool (e.g., bb8 or deadpool crates for async pooling). Ensure your application handles errors gracefully, especially around database operations.

6. Next Steps
Implement additional features for querying your journal, like getting orders by date range, by status, or by strategy.
Consider creating a dashboard or a CLI tool for easy monitoring and analysis of your trading activities.
As your requirements evolve, you may find it useful to add more fields to your database or to create additional tables for more detailed analysis (e.g., separate tables for strategies or trade outcomes).
This setup gives you a solid foundation for tracking and analyzing your orders. It can be expanded with more sophisticated querying and analysis capabilities as needed.

 */
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
