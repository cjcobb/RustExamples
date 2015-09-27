#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate threadpool;

use std::net::{TcpListener};
use std::io::Write;
use std::io::Read;

use std::str;
use std::string::String;

use threadpool::ThreadPool;

use bson::Bson;
use mongodb::{Client,ThreadedClient};
use mongodb::db::ThreadedDatabase;

const NUM_THREADS : usize = 50;
const HOST : &'static str = "0.0.0.0:9999";


//Send a key, receive all values associated with a given key in the mongo collection mydb.coll1 running on local machine
fn main() {


    let pool = ThreadPool::new(NUM_THREADS);
    
    println!("Starting server");

    //starts listening
    let listener = TcpListener::bind(HOST).unwrap();

    //connects to db
    let base_client = Client::connect("localhost",27017).ok().expect("Failed to initialize client");
    
    println!("connected to db");
    //this is just going to stay open forever, until we shut the server down
    for stream in listener.incoming() {
        //create a new reference for this thread to capture
        //props to sam rossi for this one
        let client = base_client.clone();
    	match stream {
    		Ok(stream) => {
    			//hand off to thread pool. Thread pool handles request queueing if all threads are busy
    			pool.execute(move || { 
                    let coll = client.db("mydb").collection("coll1");
    				//makes the stream mutable
    				let mut stream = stream;
    				//a mutable buffer to write the bytes into
    				let mut buf : [u8; 128] = [0 ; 128]; 
    				//returns number of bytes, which we dont care about. but reads them into the buffer
    				let _ = stream.read(&mut buf).unwrap();

                    //parses the string out of the byte buffer
                    let lookup_str : String = bytes_to_string(&buf);
    				
                    println!("message is {}",lookup_str);

                    let doc = doc ! {"key" => (lookup_str)};

                    let cursor = coll.find(Some(doc), None).ok().expect("failed to execute find");

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
		            response_msg.push_str("\n");                    

    				//send back the response
    				stream.write(response_msg.as_bytes()).unwrap();

    			});
    		}
    		Err(_) => {
    			println!("bad socket stream");
    		}
    	}
    }
    drop(listener);
}
//not needed but kept for reference
/*fn block_and_retrieve(mutex : &Arc<Mutex<VecDeque<TcpStream>>>) -> Option<TcpStream> {
    let mut vec = mutex.lock().unwrap();
    vec.pop_front()
}*/

//takes a buffer of bytes, some of which may be zero
//returns the prefix of nonzero bytes, converted into a string of chars
fn bytes_to_string(buf : &[u8]) -> String {
    let bytes = str::from_utf8(&buf).unwrap();
    bytes[0..true_len(bytes)].to_string()
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
