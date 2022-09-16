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
            println!("Connected by {}", addr);

            thread::spawn(move || {
                handle(&client);
            });
        }
    }
}

fn handle(mut client: &TcpStream) {
    let mut reader = BufReader::new(client);
    let mut buf = String::new();
    while let Ok(_) = reader.read_line(&mut buf) {
        print!("> {}", buf);
        client.write_all(buf.as_bytes()).unwrap();
        buf = String::new()
    }
}
