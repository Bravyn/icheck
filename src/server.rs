use std::net::{TcpListener, TcpStream };

pub fn handle_client(stream: TcpStream) {
    println!("Hello \U+1F445");
}

pub fn start() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}