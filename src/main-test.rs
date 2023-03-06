use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write, Read}};
use std::str;


fn handle_client(mut stream: TcpStream) {
    let response = "+PONG\r\n";

    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { 
                    // connection was closed
                    break;
                }
                println!("{:?}", read);
                println!("{}", str::from_utf8(&read).unwrap());
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}

