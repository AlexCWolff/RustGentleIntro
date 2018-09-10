// server2.rs

// Here is a more solid server that handles the error without failing. It also specifically reads a line from the stream, which is done using 'io::BufReader' to create an 'io::BufRead' on which we can call 'read_line'.

// server2.rs
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io;

fn handle_connection(stream: TcpStream) -> io::Result<()>{
    let mut rdr = io::BufReader::new(stream);
    let mut text = String::new();
    rdr.read_line(&mut text)?;
    println!("got '{}'", text.trim_right());
    Ok(())
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").expect("could not start server");

    // accept connections and get a TcpStream
    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    println!("error {:?}", e);
                }
            }
            Err(e) => { print!("connection failed {}\n", e); }
        }
    }
}
/* 'read_line' might fail in 'handle_connection', but the resulting error is safely handled. One-way communications like this are useful; for instance. a set of services across a network which want to collect their status reports together in one central place. */