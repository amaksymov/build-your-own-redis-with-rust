use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write, Read}};


fn handle_client(mut stream: TcpStream) {
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { 
                    // connection was closed
                    break;
                }
                stream.write_all(&read).unwrap();
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
