// client.rs

/* Rust provides a straightforward interface to the most commonly used network protocol, TCP. It is very fault-resistant and is the base on which our networked world is built; packets of data are sent and received, with acknowledgement. By contrast, UDP sends packets out into the wild without acknowledgement. However, error handling is very important with networking, because anything can happen, and will, eventually. TCP works as a client/server model; the server listens on a address and a particular network port, and the client connects to that server. A connection is established and thereafter the client and server can communicate with a socket.

'TcpStream::connect' takes anything that can convert into a 'SocketAddr', in particular the plain strings we have been using. A simple TCP client in Rust is easy; a 'TcpStream' struct is both readable and writeable. As usual, we have to bring the Read, Write and other 'std::io' traits into scope. */

// client.rs
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("connection failed");

    write!(stream,"hello from the client!\n").expect("write failed");
 } 