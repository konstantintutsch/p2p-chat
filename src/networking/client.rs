use log::{debug, error};

use std::net::TcpStream;
use std::io::{Result, Write, stdin};
use std::sync::Arc;

use rustls::{ClientConnection, Stream};
use rustls_platform_verifier::BuilderVerifierExt;

use crate::networking::{protocol, protocol::MessageType};

pub fn connect(target: String, name: String) {
    // Prepare TLS
    let mut tls_config = rustls::ClientConfig::builder_with_provider(rustls_openssl::default_provider().into())
        .with_safe_default_protocol_versions().unwrap()
        .with_platform_verifier().unwrap()
        .with_no_client_auth();
    tls_config.key_log = Arc::new(rustls::KeyLogFile::new());

    // Connect via TCP
    let mut tcp_stream = TcpStream::connect((target.clone(), protocol::PORT))
        .expect("Failed to connect to receiver: {error:?}");

    // TLS over TCP
    let mut tls_connection = rustls::ClientConnection::new(Arc::new(tls_config), target.try_into().unwrap())
        .expect("Failed to create TLS connection");
    let mut tls_stream = rustls::Stream::new(&mut tls_connection, &mut tcp_stream);

    // Send name
    match send(&mut tls_stream, protocol::encode_message(MessageType::Name, &name)) {
        Ok(()) => debug!("Send name"),
        Err(error) => error!("Failed to send name: {error:?}")
    }

    // Chat interface
    loop {
        let mut message = String::new();
        let message_n = stdin().read_line(&mut message)
            .expect("Failed to read message from stdin");
        message.truncate(message_n - 1); // Remove trailing \n from input

        match send(&mut tls_stream, message) {
            Ok(()) => debug!("Send message"),
            Err(error) => error!("Failed to send message: {error:?}")
        }
    }
}

fn send(stream: &mut Stream<ClientConnection, TcpStream>, mut message: String) -> Result<()> {
    message.push('\n');
    
    stream.write_all(message.as_bytes())
        .expect("Failed to send message");

    Ok(())
}
