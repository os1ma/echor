use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

pub fn run(ip_address: &String, port: &String) {
    let address = format!("{}:{}", ip_address, port);
    let server = TcpListener::bind(address).unwrap();
    server.set_nonblocking(true).unwrap();
    println!("Server started {}:{}", ip_address, port);

    loop {
        if let Ok((client, addr)) = server.accept() {
            println!("Connected by {}", addr);

            thread::spawn(|| {
                handle(client);
            });
        }
    }
}

fn handle(client: TcpStream) {
    let mut reader = BufReader::new(client);
    let mut buf = String::new();
    while let Ok(_) = reader.read_line(&mut buf) {
        print!("{}", buf);
        buf = String::new()
    }
}
