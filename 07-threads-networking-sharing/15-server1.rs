// server1.rs

// The server is not much more complicated; we set up a listener and wait for connections. When a client connects, we get a 'TcpStream' on the server side. In this case, we read everything that the client has written into a string.

// server.rs
use std::net::TcpListener;
use std::io::prelude::*;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8000").expect("could not start server");

    // accept connections and get a TcpStream
    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                let mut text = String::new();
                stream.read_to_string(&mut text).expect("read failed");
                println!("got '{}'", text);
            }
            Err(e) => { println!("connection failed {}", e); }
        }
    }
}
/* Here we've chosen a port number more or less at random, but most ports are assigned some meaning. Note that both parties have to agree on a protocol; the client expects it can write text to the stream, and the server expects to read text from the stream. If they don't, then situations can occur where one party is blocked waiting for bytes that never come. Error checking is important, network I/O can fail for many reasons, and errors that might appear once in a blue moon on a local filesystem can happen on a regular basis. This little server isn't very robust, because it will fall over on the first read error. */