use std::net::TcpStream;
use std::io::Write;

pub fn send(message: &str) -> std::io::Result<()> {
    println!("Send: {}", message);

    let mut stream = TcpStream::connect("127.0.0.1:60042")?;
    stream.write(message.as_bytes());

    Ok(())
}
