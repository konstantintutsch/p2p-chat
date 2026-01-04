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
        server::listen();
    } else if args[1] == "client" {
        let mut message = String::new();

        while message != "exit" {
            message = String::new();

            println!("\nEnter message: ");
            let message_n = io::stdin().read_line(&mut message).unwrap();
            println!("");

            // Remove trailing \n from input
            message.truncate(message_n - 1);

            client::send(&message);
        }
    } else {
        error!("Unknown option {}", args[1]);
    }
}
