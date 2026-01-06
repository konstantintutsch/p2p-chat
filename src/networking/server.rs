use std::net::{TcpListener, TcpStream};
use std::io::{Result, BufRead, BufReader};
use log::{debug};

use crate::networking::protocol;

fn handle_stream(stream: TcpStream) -> Result<()> {
    let mut name = String::from("Unknown");

    let reader = BufReader::new(stream);
    for line in reader.lines() {
        debug!("Incoming message");
        let message = line?;

        match protocol::decode_message(protocol::NAME, &message) {
            Some(new) => {
                name = new.to_string();
                debug!("Updated name: {new}");
            },
            None => println!("{name}: {message}")
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
