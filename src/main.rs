use std::env;
use std::io;

mod networking;
use networking::server;
use networking::client;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return;
    }

    if args[1] == "server" {
        server::listen();
    } else if args[1] == "client" {
        let mut message = String::new();

        while message != "exit" {
            message = String::new();

            println!("Enter message: ");
            let message_n = io::stdin().read_line(&mut message).unwrap();
            // Remove trailing \n from input
            message.truncate(message_n - 1);

            client::send(&message);
        }
    } else {
        println!("Unknown option {}", args[1]);
    }
}
