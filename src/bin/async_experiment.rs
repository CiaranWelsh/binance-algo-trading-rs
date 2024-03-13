// use std::io::{Read, Write};
// use std::net;


// // use async_std::io::prelude::*;
// use async_std::net as async_net;
// use async_std::io::Write as AsyncWrite;
//

// fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String>
// {
//     let mut socket = net::TcpStream::connect((host, port))?;
//     let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
//     socket.write_all(request.as_bytes())?;
//     socket.shutdown(net::Shutdown::Write)?;
//     let mut response = String::new();
//     socket.read_to_string(&mut response)?;
//     Ok(response)
// }


use log::Level::Error;
// use async_std::io::prelude::*;
// use async_std::net;
use tokio::{net, task};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::JoinHandle;

async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String>
{
    let mut socket = net::TcpStream::connect((host, port)).await?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown().await;
    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    Ok(response)
}


// pub async fn many_requests(requests: Vec<(String, u16, String)>) -> JoinHandle<Result<String>>
// {
//     let mut handles = vec![];
//     for (host, port, path) in requests {
//         handles.push(task::spawn_local(cheapo_request(&host,
//                                                       port, &path)));
//     }
//     let mut results = vec![];
//     for handle in handles {
//         results.push(handle.await);
//     }
//     results
// }


#[tokio::main]
async fn main() {
    // Adjust the host, port, and path as necessary
    let host = "127.0.0.1";
    let port = 8000;
    let path = "/";

    match cheapo_request(host, port, path).await {
        Ok(response) => println!("Received response: \n{}", response),
        Err(e) => println!("Failed to make request: {}", e),
    }
}

