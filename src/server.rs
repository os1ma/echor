use std::{
    io::{BufRead, BufReader, Write},
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
            thread::spawn(move || {
                println!("Connected by {}", addr);
                handle(&client);
                println!("Connection closed {}", addr);
            });
        }
    }
}

fn handle(mut client: &TcpStream) {
    let mut reader = BufReader::new(client);
    let mut buf = String::new();
    while let Ok(size) = reader.read_line(&mut buf) {
        if size == 0 {
            break;
        }

        print!("> {}", buf);
        client.write_all(buf.as_bytes()).unwrap();
        buf = String::new()
    }
}
