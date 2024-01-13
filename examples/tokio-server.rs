use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time;

const ADDRESS: &str = "127.0.0.1:7878";

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(ADDRESS).await.unwrap();
    println!("{ADDRESS}");

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(stream).await
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line =  buf_reader.lines().next_line().await.unwrap().unwrap();
    let (status_line, contents) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", include_str!("./serve.html")),
        "GET /sleep HTTP/1.1" => {
            time::sleep(Duration::from_secs(100)).await;
            ("HTTP/1.1 200 OK", include_str!("./serve.html"))
        }
        "GET /error HTTP/1.1" => {
            panic!()
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "nothing"),
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).await.unwrap();
}
