mod client;
mod server;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprint_usage();
        return;
    }

    let behavior_type = &args[1];
    let ip_address = &args[2];
    let port = &args[3];

    match behavior_type.as_str() {
        "server" => server::run(ip_address, port),
        "client" => client::run(ip_address, port),
        _ => {
            eprintln!("Invalid argument. You must specify server or client.");
            eprint_usage();
            process::exit(1)
        }
    }
}

fn eprint_usage() {
    eprintln!("Usage: echor <server|client> <ip address> <port>")
}
