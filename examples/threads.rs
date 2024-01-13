use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

const ADDRESS: &str = "127.0.0.1:7878";
fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();
    println!("THREADED: {ADDRESS}");
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {handle_connection(stream);});
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, contents) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", include_str!("./serve.html")),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(std::time::Duration::from_secs(100));
            ("HTTP/1.1 200 OK", include_str!("./serve.html"))
        }
        "GET /error HTTP/1.1" => {
            panic!()
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "nothing"),
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
