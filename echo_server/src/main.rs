use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Read;
use std::str;
use std::thread;


fn main() {
    println!("Starting server");

    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    for stream in listener.incoming() {
    	match stream {
    		Ok(stream) => {
    			thread::spawn(|| {
    				let mut stream = stream;
    				let mut buf : [u8; 128] = [0 ; 128]; 
    				let _ = stream.read(&mut buf).unwrap();
    				println!("message is {}",str::from_utf8(&buf).unwrap());
    				thread::sleep_ms(5000);
    				stream.write(b"yea, I got your message").unwrap();

    			});
    		}
    		Err(_) => {
    			println!("fuck");
    		}
    	}
    }
    drop(listener);
}
