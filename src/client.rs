use std::{
    io::{stdin, BufRead, BufReader, Write},
    net::TcpStream,
};

pub fn run(ip_address: &String, port: &String) {
    let address = format!("{}:{}", ip_address, port);
    println!("Connecting to {}:{}", ip_address, port);

    let mut socket = &TcpStream::connect(address).unwrap();
    let mut reader = BufReader::new(socket);

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        socket.write(input.as_bytes()).unwrap();

        let mut buf = String::new();
        if let Ok(_) = reader.read_line(&mut buf) {
            print!("< {}", buf);
        }
    }
}
