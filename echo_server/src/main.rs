#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

use std::net::{TcpListener};
use std::io::Write;
use std::io::Read;
use std::str;
use std::thread;

use std::string::String;

use bson::Bson;
use mongodb::{Client,ThreadedClient};
use mongodb::db::ThreadedDatabase;


//Send a key, receive all values associated with a given key in the mongo collection mydb.coll1 running on local machine
fn main() {
    println!("Starting server");

    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    //this is just going to stay open forever, until we shut the server down
    for stream in listener.incoming() {
    	match stream {
    		Ok(stream) => {
    			//this is not necessary, but just shows a pattern
    			thread::spawn(|| {
                    //should have these outside of thread
                    let client = Client::connect("localhost",27017).ok().expect("Failed to initialize client");
                    let coll = client.db("mydb").collection("coll1");

    				//makes the stream mutable
    				let mut stream = stream;
    				//a mutable buffer to write the bytes into
    				let mut buf : [u8; 128] = [0 ; 128]; 
    				//returns number of bytes, which we dont care about. but reads them into the buffer
    				let _ = stream.read(&mut buf).unwrap();
    				//bytes to string conversion. This array of bytes has a bunch of zeroes though
                    let lookup_str = str::from_utf8(&buf).unwrap();
    				
                    println!("message is {}, length of message is {}",&lookup_str[0..4], lookup_str.len());

                    //need to throw out those zeroes
                    let doc = doc ! {"key" => (&lookup_str[0..true_len(lookup_str)])};

                    //note the clone, so we dont run into problems with the borrower
                    let cursor = coll.find(Some(doc.clone()), None).ok().expect("failed to execute find");

                    //get all the values associated with a given key
                    let mut response_msg : String = "".to_string();
                    for item in cursor {
                        match item {
                            Ok(doc) => match doc.get("value") {
                                Some(&Bson::String(ref val)) => {
                                    response_msg.push_str(val); 
                                    response_msg.push_str(" ");},
                                _ => println!("Error, improper field"),
                            },
                            Err(_) => println!("Error fetching document"),
                        }
                    }                    

    				//send back the response
    				stream.write(response_msg.as_bytes()).unwrap();

    			});
    		}
    		Err(_) => {
    			println!("fuck");
    		}
    	}
    }
    drop(listener);
}

//finds the index of the last non null byte in a string
fn true_len(x : &str) -> usize {
    let mut i = 0;
    for c in x.as_bytes() {
        if *c == 0 {
            break;
        }
        else {
            i = i + 1;
        }
    }
    i
}
