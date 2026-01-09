use std::panic;
use std::{io, thread};
use log::{debug, error};

mod networking;
use networking::server;
use networking::client;

fn main() {
    env_logger::init();

    let mut target = String::new();
    let mut name = String::new();

    thread::spawn(|| {
        let listen_result = server::listen();
        match listen_result {
            Ok(_) => debug!("Listened successfully"),
            Err(error) => error!("Failed to listen: {error:?}")
        };
    });

    println!("Connect to [host]:");
    let target_result = io::stdin().read_line(&mut target);
    let target_n = match target_result {
        Ok(n) => n,
        Err(error) => panic!("Failed to read target from stdin: {error:?}")
    };
    target.truncate(target_n - 1); // Remove trailing \n from input

    println!("Connect as [name]:");
    let name_result = io::stdin().read_line(&mut name);
    let name_n = match name_result {
        Ok(n) => n,
        Err(error) => panic!("Failed to read name from stdin: {error:?}")
    };
    name.truncate(name_n - 1); // Remove trailing \n from input

    client::connect(target, name);
}
