use std::net::TcpStream;
use std::io::Write;
use log::{debug, error};

pub fn send(message: &str) {
    debug!("Sending message: {message}");

    let connect_result = TcpStream::connect("127.0.0.1:60042");
    let mut stream = match connect_result {
        Ok(stream) => stream,
        Err(error) => {
            error!("Failed to connect to receiver: {error:?}");
            return;
        }
    };

    let write_result = stream.write(message.as_bytes());
    match write_result {
        Ok(n_bytes) => debug!("Wrote {n_bytes} bytes to TCP stream"),
        Err(error) => error!("Failed to write to stream: {error:?}")
    };
}
