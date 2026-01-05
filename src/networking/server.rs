use std::net::{TcpListener, TcpStream};
use std::io::Read;
use log::{debug, error};

fn handle_package(mut stream: TcpStream) {
    let mut buffer: String = Default::default();
    let read_result = stream.read_to_string(&mut buffer);

    match read_result {
        Ok(n_bytes) => {
            debug!("Read {n_bytes} bytes from TCP stream");
            println!("Received: {buffer}");
        },
        Err(error) => error!("Failed to read from stream: {error:?}")
    };
}

pub fn listen(host: String) {
    let bind_result = TcpListener::bind(host);
    let listener = match bind_result {
        Ok(listener) => listener,
        Err(error) => {
            error!("Failed to bind listener: {error:?}");
            return;
        }
    };
    debug!("Listening on {}", listener.local_addr().unwrap());

    for incoming_result in listener.incoming() {
        debug!("Incoming TCP stream");

        match incoming_result {
            Ok(stream) => handle_package(stream),
            Err(error) => error!("Error receiving stream: {error:?}")
        };
    }
}
