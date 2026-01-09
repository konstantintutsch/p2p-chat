use std::net::TcpStream;
use std::io::{Result, Write, stdin};
use std::panic;
use log::{debug, error};

use crate::networking::{protocol, protocol::MessageType};

pub fn connect(target: String, name: String) {
    let connect_result = TcpStream::connect((target, protocol::PORT));
    let stream = match connect_result {
        Ok(stream) => stream,
        Err(error) => {
            error!("Failed to connect to receiver: {error:?}");
            return;
        }
    };

    let name_result = send(&stream, &protocol::encode_message(MessageType::Name, &name));
    match name_result {
        Ok(_) => debug!("Send name: {name}"),
        Err(error) => error!("Failed to send name: {error:?}")
    }

    loop {
        let mut message = String::new();

        let message_result = stdin().read_line(&mut message);
        let message_n = match message_result {
            Ok(n) => n,
            Err(error) => panic!("Failed to read message from stdin: {error:?}")
        };
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
