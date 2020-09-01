mod parser;

use parser::Parser;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    println!("{}", str::from_utf8(&buffer).unwrap());
    let parser = Parser::new(str::from_utf8(&buffer).unwrap());
    println!("{:?}", parser);

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();

        // let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8089")?;
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
    Ok(())
}
