use std::panic;
use std::{io, thread};
use log::{debug, error};

mod networking;
use networking::server;
use networking::client;

fn main() {
    env_logger::init();

    let mut host = String::new();

    let mut target = String::new();
    let mut name = String::new();

    println!("Listen [host:port]:");
    let host_result = io::stdin().read_line(&mut host);
    let host_n = match host_result {
        Ok(n) => n,
        Err(error) => panic!("Failed to read host from stdin: {error:?}")
    };
    host.truncate(host_n - 1); // Remove trailing \n from input

    thread::spawn(|| {
        let listen_result = server::listen(host);
        match listen_result {
            Ok(_) => debug!("Listened successfully"),
            Err(error) => error!("Failed to listen: {error:?}")
        };
    });

    println!("Connect [host:port]:");
    let target_result = io::stdin().read_line(&mut target);
    let target_n = match target_result {
        Ok(n) => n,
        Err(error) => panic!("Failed to read target from stdin: {error:?}")
    };
    target.truncate(target_n - 1); // Remove trailing \n from input

    println!("Name:");
    let name_result = io::stdin().read_line(&mut name);
    let name_n = match name_result {
        Ok(n) => n,
        Err(error) => panic!("Failed to read name from stdin: {error:?}")
    };
    name.truncate(name_n - 1); // Remove trailing \n from input

    client::connect(target, name);
}
