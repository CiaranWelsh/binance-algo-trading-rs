use std::fmt::format;
use std::sync::{Arc};
use rust_decimal::Decimal;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_postgres::{NoTls, Error, Client};
use std::io::{Error as IOError, ErrorKind};
use log::trace;
use crate::binance_api::streams::kline_data::{Kline, KlineMessage};

#[derive(Debug)]
pub struct DatabaseClient {
    pub client: Client,
    connection_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}


impl DatabaseClient {
    pub async fn new(dbname: &str, user: &str, pwd: &str) -> Result<Self, Error> {
        // Try connecting directly first
        let direct_conn_str = format!("host=localhost user={} password={} dbname={}", user, pwd, dbname);
        if let Ok((client, connection)) = tokio_postgres::connect(&direct_conn_str, NoTls).await {
            let connection_handle = tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            return Ok(DatabaseClient {
                client,
                connection_handle: Arc::new(Mutex::new(Some(connection_handle))),
            });
        }

        // If direct connection fails, proceed with connect_and_setup to ensure the database is created
        Self::connect_or_create_if_not_exist(dbname, user, pwd).await
    }

    // Connects to the default database to check for the existence of the target database
    pub async fn connect_or_create_if_not_exist(dbname: &str, user: &str, password: &str) -> Result<Self, Error> {
        let admin_conn_str = format!("host=localhost user={} password={} dbname=postgres", user, password);
        let (admin_client, connection) = tokio_postgres::connect(&admin_conn_str, NoTls).await?;

        // Spawning a new task for the connection to keep it alive
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        // Check if the target database exists
        let row = admin_client.query_one("SELECT 1 FROM pg_database WHERE datname = $1", &[&dbname]).await;
        if row.is_err() {
            // If the database does not exist, create it
            admin_client.execute(&format!("CREATE DATABASE {}", dbname), &[]).await?;
        }

        // Now connect to the newly created or existing database
        let db_conn_str = format!("host=localhost user={} password={} dbname={}", user, password, dbname);
        Self::connect(&db_conn_str).await
    }

    // Method to check if the specified database exists
    pub async fn database_exists(dbname: &str, user: &str, password: &str) -> Result<bool, IOError> {
        let (client, connection) = match tokio_postgres::connect(format!("host=localhost user={} password={} dbname=postgres", user, password).as_str(), NoTls).await {
            Ok(conn) => conn,
            Err(e) => return Err(IOError::new(ErrorKind::ConnectionRefused, e.to_string())),
        };

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let stmt = "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)";
        match client.query_one(stmt, &[&dbname]).await {
            Ok(row) => Ok(row.get(0)),
            Err(e) => Err(IOError::new(ErrorKind::Other, e.to_string())),
        }
    }

    pub async fn connect(connection_string: &str) -> Result<Self, Error> {
        let (client, connection) =
            tokio_postgres::connect(connection_string, NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        let handle = tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(DatabaseClient {
            client,
            connection_handle: Arc::new(Mutex::new(Some(handle))),
        })
    }

    pub async fn close(&self) {
        let mut handle = self.connection_handle.lock();
        if let Some(h) = handle.await.take() {
            h.abort();
        }
    }

    // Method to drop a database
    pub async fn drop_database_if_exists(db_name: &str, user: &str, password: &str) -> Result<(), Error> {
        // Form the connection string to the default or administration database
        let conn_str = format!("host=localhost user={} password={} dbname=postgres", user, password);

        // Establish a temporary connection
        let (client, connection) = tokio_postgres::connect(&conn_str, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        // Formulate and execute the drop database command
        let stmt = format!("DROP DATABASE IF EXISTS {}", db_name);
        client.execute(&stmt, &[]).await?;

        Ok(())
    }


    // This method ensures the kline_data table exists
    pub async fn ensure_kline_data_table_exists(&self) -> Result<(), Error> {
        let create_table_stmt = "
            CREATE TABLE IF NOT EXISTS kline_data (
                symbol VARCHAR(20),
                interval VARCHAR(10),
                start_time BIGINT,
                end_time BIGINT,
                open_price DOUBLE PRECISION,
                close_price DOUBLE PRECISION,
                high_price DOUBLE PRECISION,
                low_price DOUBLE PRECISION,
                base_asset_volume DOUBLE PRECISION,
                number_of_trades INT,
                is_kline_closed BOOLEAN,
                quote_asset_volume DOUBLE PRECISION,
                taker_buy_base_asset_volume DOUBLE PRECISION,
                taker_buy_quote_asset_volume DOUBLE PRECISION,
                PRIMARY KEY (symbol, interval, start_time)
            );
        ";

        self.client.execute(create_table_stmt, &[]).await?;

        Ok(())
    }

    pub async fn insert_kline_data(&self, kline_message: &KlineMessage) -> Result<(), Error> {
        let kline = &kline_message.data.k;

        let stmt = "
    INSERT INTO kline_data (
        symbol, interval, start_time, end_time, open_price, close_price,
        high_price, low_price, base_asset_volume, number_of_trades,
        is_kline_closed, quote_asset_volume, taker_buy_base_asset_volume,
        taker_buy_quote_asset_volume
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
    ON CONFLICT (symbol, interval, start_time)
    DO UPDATE SET
        end_time = EXCLUDED.end_time,
        open_price = EXCLUDED.open_price,
        close_price = EXCLUDED.close_price,
        high_price = EXCLUDED.high_price,
        low_price = EXCLUDED.low_price,
        base_asset_volume = EXCLUDED.base_asset_volume,
        number_of_trades = EXCLUDED.number_of_trades,
        is_kline_closed = EXCLUDED.is_kline_closed,
        quote_asset_volume = EXCLUDED.quote_asset_volume,
        taker_buy_base_asset_volume = EXCLUDED.taker_buy_base_asset_volume,
        taker_buy_quote_asset_volume = EXCLUDED.taker_buy_quote_asset_volume
    ";

        self.client.execute(stmt, &[
            &kline_message.data.symbol,
            &kline.interval,
            &(kline.start_time as i64),
            &(kline.end_time as i64),
            &kline.open_price,
            &kline.close_price,
            &kline.high_price,
            &kline.low_price,
            &kline.base_asset_volume,
            &(kline.number_of_trades as i32),
            &kline.is_kline_closed,
            &kline.quote_asset_volume,
            &kline.taker_buy_base_asset_volume,
            &kline.taker_buy_quote_asset_volume,
        ]).await?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use chrono::Month::December;
    use log::LevelFilter::Trace;
    use log::trace;
    use rust_decimal::Decimal;
    use serde::{Deserialize, Serialize};
    use super::DatabaseClient;
    use tokio_postgres::NoTls;
    use crate::binance_api::load_env::EnvVars;
    use crate::binance_api::logger_conf::init_logger;
    use crate::binance_api::streams::kline_data::{Kline, KlineData, KlineMessage};

    #[tokio::test]
    async fn test_database_creation() {
        init_logger(Trace);
        // Setup unique test database parameters
        let vars = EnvVars::new();
        let db_name = vars.name.as_str();
        let user = vars.user.as_str();
        let pwd = vars.pwd.as_str();

        trace!("vars: {:?}", vars);

        // Attempt to create the database
        let db_client = DatabaseClient::connect_or_create_if_not_exist(&db_name, &user, &pwd).await.expect("Failed to setup database");

        // Verify the database exists
        let exists = DatabaseClient::database_exists(&db_name, &user, &pwd).await.expect("Failed to check if database exists");
        assert!(exists, "Database was not created successfully");

        db_client.close().await;

        // Drop the database to clean up
        DatabaseClient::drop_database_if_exists(&db_name, &user, &pwd).await.expect("Failed to drop the test database");

        // Verify the database no longer exists
        let exists = DatabaseClient::database_exists(&db_name, &user, &pwd).await.expect("Failed to check if database exists after dropping");
        assert!(!exists, "Database was not dropped successfully");
    }

    #[tokio::test]
    async fn test_database_operations_with_kline_data() {
        let vars = EnvVars::new();

        let database_client = DatabaseClient::connect_or_create_if_not_exist(vars.name.as_str(), vars.user.as_str(), vars.pwd.as_str()).await.expect("Failed to setup database");

        // Ensure kline_data table exists
        database_client.ensure_kline_data_table_exists().await.expect("Failed to ensure kline_data table exists");

        // Directly create a KlineMessage for testing
        let kline_message = KlineMessage {
            stream: "btcusdt@kline_1m".to_string(),
            data: KlineData {
                event_type: "kline".to_string(),
                event_time: 1609459200000,
                symbol: "BTCUSDT".to_string(),
                k: Kline {
                    start_time: 1609459200000,
                    end_time: 1609459260000,
                    symbol: "BTCUSDT".to_string(),
                    interval: "1m".to_string(),
                    first_trade_id: 100,
                    last_trade_id: 105,
                    open_price: 29000.1,
                    close_price: 29001.0,
                    high_price: 29005.5,
                    low_price: 28995.5,
                    base_asset_volume: 100.5,
                    number_of_trades: 100,
                    is_kline_closed: true,
                    quote_asset_volume: 2910000.5,
                    taker_buy_base_asset_volume: 50.25,
                    taker_buy_quote_asset_volume: 1455000.25,
                    ignore: "0".to_string(),
                },
            },
        };

        // Insert the Kline data
        database_client.insert_kline_data(&kline_message).await.expect("Failed to insert kline data");

        // Query and validate the inserted data
        let rows = database_client.client.query("SELECT symbol, interval, start_time, end_time, open_price FROM kline_data WHERE symbol = $1", &[&kline_message.data.symbol]).await.expect("Failed to query kline data");
        assert!(!rows.is_empty(), "No data found after insertion");

        database_client.close().await;
        drop(database_client);

        // Clean up by dropping the test database
        // database_client.execute(&*format!("DROP DATABASE {}", db_name), &[]).await.expect("Failed to drop test database after tests");

        DatabaseClient::drop_database_if_exists(vars.name.as_str(), vars.user.as_str(), vars.pwd.as_str()).await.expect("Failed to drop database");
    }
}

