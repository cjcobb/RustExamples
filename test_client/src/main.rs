use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::str;

fn main() {
    println!("Hello, world!");

    let mut stream = TcpStream::connect("127.0.0.1:9999").unwrap();
    let _ = stream.write(b"foo");
    let mut buf : [u8; 128] = [0; 128];
    let _ = stream.read(&mut buf).unwrap();
    let foo = str::from_utf8(&buf).unwrap();
    println!("Recieved {}", foo);
}
