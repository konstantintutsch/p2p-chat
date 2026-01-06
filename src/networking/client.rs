use std::net::TcpStream;
use std::io::{Result, Write, stdin};
use log::{debug, error};

pub fn connect(target: String) {
    let connect_result = TcpStream::connect(target);
    let stream = match connect_result {
        Ok(stream) => stream,
        Err(error) => {
            error!("Failed to connect to receiver: {error:?}");
            return;
        }
    };

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
