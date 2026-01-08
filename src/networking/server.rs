use std::net::{TcpListener, TcpStream};
use std::io::{Result, BufRead, BufReader};
use log::{debug};

use crate::networking::{protocol, protocol::MessageType};

fn handle_stream(stream: TcpStream) -> Result<()> {
    let mut name = String::from("Unknown");

    let reader = BufReader::new(stream);
    for line in reader.lines() {
        debug!("Incoming message");
        let message = line?;

        match protocol::get_message_type(&message) {
            MessageType::Name => name = protocol::decode_message(MessageType::Name, &message),
            _ => println!("{name}: {message}")
        }        
    }

    Ok(())
}

pub fn listen(host: String) -> Result<()> {
    let listener = TcpListener::bind(host)?;
    debug!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        debug!("Incoming TCP stream");
        handle_stream(stream?)?;
    }

    Ok(())
}
