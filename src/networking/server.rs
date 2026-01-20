use log::{debug};

use std::net::{TcpListener, TcpStream};
use std::io::{Result, BufRead};
use std::sync::Arc;

use rustls::{ServerConnection, Stream};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::pki_types::pem::PemObject;

use crate::networking::{protocol, protocol::MessageType};

fn handle_stream(stream: Stream<ServerConnection, TcpStream>) -> Result<()> {
    let mut name = String::from("Unknown");

    for line in stream.lines() {
        debug!("Incoming message");
        let message = line.expect("Failed to get message from stream");

        match protocol::get_message_type(&message) {
            MessageType::Name => name = protocol::decode_message(MessageType::Name, &message),
            _ => println!("{name}: {message}")
        }        
    }

    Ok(())
}

pub fn listen(certificate_path: String, private_key_file: String) -> Result<()> {
    // Load certificate and private key
    let certificates = CertificateDer::pem_file_iter(certificate_path)
        .expect("Failed to load certificate from file")
        .map(|certificate| certificate.unwrap())
        .collect();

    let private_key = PrivateKeyDer::from_pem_file(private_key_file)
        .expect("Failed to load private key from file");

    // Prepare TLS
    let tls_config = rustls::ServerConfig::builder_with_provider(rustls_openssl::default_provider().into())
        .with_safe_default_protocol_versions().unwrap()
        .with_no_client_auth()
        .with_single_cert(certificates, private_key)
        .expect("Failed to create rustls::ServerConfig");

    // TLS over TCP
    let tcp_listener = TcpListener::bind(("0.0.0.0", protocol::PORT))
        .expect("Failed to bind listener to port");
    let (mut tcp_stream, _) = tcp_listener.accept().unwrap();
    debug!("Listening on {}", tcp_listener.local_addr().unwrap());

    let mut tls_connection = rustls::ServerConnection::new(Arc::new(tls_config))
        .expect("Failed to create rustls::ServerConnection");

    let stream = rustls::Stream::new(&mut tls_connection, &mut tcp_stream);

    // Handle incoming messages
    handle_stream(stream)
        .expect("Failed to handle TLS stream");

    Ok(())
}
