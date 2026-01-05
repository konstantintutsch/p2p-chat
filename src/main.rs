use std::{io, thread};

mod networking;
use networking::server;
use networking::client;

fn main() {
    env_logger::init();

    let mut host = String::new();
    let mut target = String::new();

    println!("Listen on [host:port]:");
    let host_n = io::stdin().read_line(&mut host).unwrap();
    host.truncate(host_n - 1); // Remove trailing \n from input

    thread::spawn(|| {
        server::listen(host);
    });

    println!("Send to [host:port]:");
    let target_n = io::stdin().read_line(&mut target).unwrap();
    target.truncate(target_n - 1); // Remove trailing \n from input

    loop {
        let mut message = String::new();

        let message_n = io::stdin().read_line(&mut message).unwrap();
        message.truncate(message_n - 1); // Remove trailing \n from input

        client::send(&target, &message);
    }
}
