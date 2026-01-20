use std::{io, panic, thread, env};
use log::{debug, error};

mod networking;
use networking::{client, server};

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let certificate_path = args[1].clone();
    let privatekey_path = args[2].clone();

    thread::spawn(|| {
        let listen_result = server::listen(certificate_path, privatekey_path);
        match listen_result {
            Ok(_) => debug!("Listened successfully"),
            Err(error) => error!("Failed to listen: {error:?}")
        };
    });

    let mut target = String::new();
    let mut name = String::new();

    println!("Connect to [host]:");
    let target_result = io::stdin().read_line(&mut target);
    let target_n = match target_result {
        Ok(n) => n,
        Err(error) => panic!("Failed to read target from stdin: {error:?}")
    };

    println!("Connect as [name]:");
    let name_result = io::stdin().read_line(&mut name);
    let name_n = match name_result {
        Ok(n) => n,
        Err(error) => panic!("Failed to read name from stdin: {error:?}")
    };

    // Remove trailing \n from input
    target.truncate(target_n - 1);
    name.truncate(name_n - 1);

    client::connect(target, name);
}
