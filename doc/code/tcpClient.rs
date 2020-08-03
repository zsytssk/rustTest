use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:80").unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
}
