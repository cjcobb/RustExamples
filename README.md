This is a multithreaded server that uses a threadpool. Server listens for requests that are simple strings  
The strings are treated as keys, used as lookup keys in mongodb running on local machine.  
See src/main.rs for more details

These are built with cargo. You should follow the first few pages of the guide to set up rust and cargo (a build tool):   
https://doc.rust-lang.org/book/getting-started.html

TODO:
1. Define functionality that is actually needed  
	1a. Write stubs for said functionality  
2. Define database schema (even tho mongo doesnt enforce a schema, we need to keep track of one)  
3. Make server fault tolerant (i.e. bad requests dont crash server)  
