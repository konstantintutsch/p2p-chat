use std::{io, env};
use log::error;

mod networking;
use networking::server;
use networking::client;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        error!("Not enough arguments passed. Please choose: server, client");
        return;
    }

    if args[1] == "server" {
        let mut host = String::new();

        println!("Listen on [host:port]:");
        let host_n = io::stdin().read_line(&mut host).unwrap();
        host.truncate(host_n - 1); // Remove trailing \n from input

        server::listen(host);
    } else if args[1] == "client" {
        let mut target = String::new();

        println!("Send to [host:port]:");
        let target_n = io::stdin().read_line(&mut target).unwrap();
        target.truncate(target_n - 1); // Remove trailing \n from input

        loop {
            let mut message = String::new();

            println!("Enter message: ");
            let message_n = io::stdin().read_line(&mut message).unwrap();
            message.truncate(message_n - 1); // Remove trailing \n from input

            client::send(&target, &message);
        }
    } else {
        error!("Unknown option: {}", args[1]);
    }
}
