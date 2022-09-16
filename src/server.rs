use std::{
    io::{BufRead, BufReader},
    net::TcpListener,
};

pub fn run(ip_address: &String, port: &String) {
    println!("Server starting {}:{}", ip_address, port);

    let address = format!("{}:{}", ip_address, port);

    let server = TcpListener::bind(address).unwrap();
    server.set_nonblocking(true).unwrap();

    loop {
        if let Ok((client, addr)) = server.accept() {
            println!("Connected by {}", addr);

            let mut reader = BufReader::new(client);
            let mut buf = String::new();
            while let Ok(_) = reader.read_line(&mut buf) {
                println!("{}", buf);
            }
        }
    }
}
