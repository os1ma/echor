use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        print_usage();
        return;
    }

    println!("Hello, world!");
}

fn print_usage() {
    println!("Usage: echo <server|client> <ip address> <port>")
}
