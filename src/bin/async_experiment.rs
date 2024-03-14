// use tokio_postgres::{NoTls, Error};
//
// async fn insert_f64_value(uri: &str, value: f64) -> Result<(), Error> {
//     // Connect to the database
//     let (client, connection) = tokio_postgres::connect(uri, NoTls).await?;
//
//     // The connection object performs the actual communication with the database,
//     // so spawn it off to run on its own.
//     tokio::spawn(async move {
//         if let Err(e) = connection.await {
//             eprintln!("connection error: {}", e);
//         }
//     });
//
//     // Prepare your SQL statement
//     let stmt = "INSERT INTO FloatContainer (value) VALUES ($1)";
//
//     // Execute the statement with your f64 value
//     client.execute(stmt, &[&value]).await?;
//
//     Ok(())
// }
//
#[tokio::main]
async fn main() {
//     let database_url = "postgres://test:1@localhost/newdb";
//     let my_value: f64 = 123.456; // Example f64 value
//
//     match insert_f64_value(database_url, my_value).await {
//         Ok(_) => println!("Value inserted successfully."),
//         Err(e) => eprintln!("Error inserting value: {}", e),
//     }
}
