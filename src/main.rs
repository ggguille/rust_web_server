use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filepath) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "res/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "res/404.html")
    };

    let contents = fs::read_to_string(filepath).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
