use std::{io, thread};
use log::{debug, error};

mod networking;
use networking::server;
use networking::client;

fn main() {
    env_logger::init();

    let mut host = String::new();
    let mut target = String::new();

    println!("Listen [host:port]:");
    let host_n = io::stdin().read_line(&mut host).unwrap();
    host.truncate(host_n - 1); // Remove trailing \n from input

    thread::spawn(|| {
        let listen_result = server::listen(host);
        match listen_result {
            Ok(_) => debug!("Listened successfully"),
            Err(error) => error!("Failed to listen: {error:?}")
        };
    });

    println!("Connect [host:port]:");
    let target_n = io::stdin().read_line(&mut target).unwrap();
    target.truncate(target_n - 1); // Remove trailing \n from input

    client::connect(target);
}
