use std::{fs, task::Context};
use std::time::Duration;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, spawn};


#[tokio::main]
async fn main() 
{
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (socket, s) = listener.accept().await.unwrap();
        println!("Accepted response from {}", s.to_string());

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream) 
{
    let mut buffer = [0; 1024];
    let read = stream.read(&mut buffer).await.unwrap();
    assert!(read > 0);
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
    } else if buffer.starts_with(sleep) {
        tokio::time::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
