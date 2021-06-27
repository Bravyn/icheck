use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream };

fn main() {
    println!("Hello, world!{}");
    start();
}


pub fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 4096];

    stream.read(&mut buf).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

pub fn start() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
    Ok(())
}
