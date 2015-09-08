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
    			//this is not necessary, but just shows a pattern
    			thread::spawn(|| {
    				//makes the stream mutable
    				let mut stream = stream;
    				//a mutable buffer to write the bytes into
    				let mut buf : [u8; 128] = [0 ; 128]; 
    				//returns number of bytes, which we dont care about
    				let _ = stream.read(&mut buf).unwrap();
    				//bytes to string conversion
    				println!("message is {}",str::from_utf8(&buf).unwrap());
    				//just used to show multithreading
    				thread::sleep_ms(5000);
    				//note the b
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
