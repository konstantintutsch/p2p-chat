use std::net::TcpStream;
use std::io::{Result, Write, stdin};
use log::{debug, error};

use crate::networking::protocol;

pub fn connect(target: String, name: String) {
    let connect_result = TcpStream::connect(target);
    let stream = match connect_result {
        Ok(stream) => stream,
        Err(error) => {
            error!("Failed to connect to receiver: {error:?}");
            return;
        }
    };

    let name_result = send(&stream, &protocol::encode_message(protocol::NAME, &name));
    match name_result {
        Ok(_) => debug!("Set name: {name}"),
        Err(error) => error!("Failed to set name: {error:?}")
    }

    loop {
        let mut message = String::new();

        let message_n = stdin().read_line(&mut message).unwrap();
        message.truncate(message_n - 1); // Remove trailing \n from input

        let send_result = send(&stream, &message);
        match send_result {
            Ok(_) => debug!("Send message: {message}"),
            Err(error) => error!("Failed to send message: {error:?}")
        };
    }
}

fn send(mut stream: &TcpStream, message: &String) -> Result<()> {
    stream.write(message.as_bytes())?;
    stream.write(b"\n")?;

    Ok(())
}
