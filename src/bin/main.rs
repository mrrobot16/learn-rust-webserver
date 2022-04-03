use std::thread;
use std::time::Duration;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use my_first_server::{ThreadPool};

fn main() {
    println!("Hello, world my first WebServer in rust");
    init();
}

fn init() {
    let base_url = "127.0.0.1";
    let port = "7878";
    // Only admin can call using this port. 
    // let port = "80";
    let url = format!("{}:{}", base_url, port);
    
    let listener = TcpListener::bind(url).unwrap();
    
    let pool = ThreadPool::new(4);
    
    for (index, stream) in listener.incoming().enumerate() {
        let stream = stream.unwrap();
        println!("Connection #{} establised!", index);
        
        let closure = || {
            handle_connection_with_sleep(stream)
        };
        pool.execute(closure)
        
        // thread::spawn(|| {
            // handle_connection(stream);
            // handle_connection_sleep(stream);
        // });
    }
    println!("Shutting down.");
}

fn handle_connection_with_sleep(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // requests
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // responses
    let ok = ("HTTP/1.1 200 OK", "hello.html");
    let not_found = ("HTTP/1.1 404 NOT FOUND", "404.html");
    
    let( status_line, filename) = if buffer.starts_with(get) {
        // println!("ok");
        ok
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        // println!("sleep");
        ok
    } else {
        // println!("not_found");
        not_found
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    
    // stream."flush" will wait and prevent the program from continuing until all the bytes are written to the connection; 
    // TcpStream contains an internal buffer to minimize calls to the underlying operating system.
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
