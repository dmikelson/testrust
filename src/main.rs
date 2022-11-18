use std::net::{TcpListener, TcpStream};
use std::io::Read;
const MESSAGE_SIZE: usize = 128;

fn handle_client(stream: &mut TcpStream){
    // ...
    let mut received: Vec<u8> = vec![];
    let mut rx_bytes = [0u8; MESSAGE_SIZE];
    loop {
        // Read from the current data in the TcpStream
        let bytes_read = stream.read(&mut rx_bytes);
        println!("Char: {}", std::str::from_utf8(&rx_bytes[..bytes_read.unwrap()]).unwrap());
        
    }
    
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
       handle_client(&mut stream?);

    }
    Ok(())
}