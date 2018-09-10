// client-echo.rs

/* A  simple example is a basic 'echo' server. The client writes some text ending in a newline to the server, and receives the same text back with a newline - the stream is readable and writeable.

// client_echo.rs
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").expect("connection failed");
    let msg = "hello from the client!";

    write!(stream,"{}\n", msg).expect("write failed");

    let mut resp = String::new();
    stream.read_to_string(&mut resp).expect("read failed");
    let text = resp.trim_right();
    assert_eq!(msg,text);
}
The server has an interesting twist. Only handle_connection changes:

fn handle_connection(stream: TcpStream) -> io::Result<()>{
    let mut ostream = stream.try_clone()?;
    let mut rdr = io::BufReader::new(stream);
    let mut text = String::new();
    rdr.read_line(&mut text)?;
    ostream.write_all(text.as_bytes())?;
    Ok(())
}
This is a common gotcha with simple two-way socket communication; we want to read a line, so need to feed the readable stream to BufReader - but it consumes the stream! So we have to clone the stream, creating a new struct which refers to the same underlying socket. Then we have happiness. */