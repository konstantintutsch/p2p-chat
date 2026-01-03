use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn handle_package(mut stream: TcpStream) {
    let mut buffer: String = Default::default();
    stream.read_to_string(&mut buffer);

    println!("Received: {}", buffer);
}

pub fn listen() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:60042")?;

    for stream in listener.incoming() {
        handle_package(stream?);
    }
    Ok(())
}
