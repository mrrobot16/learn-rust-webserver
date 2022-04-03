// use std::io::prelude::*;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

pub fn init() {
    let base_url = "127.0.0.1";
    let port = "7878";
    // Only admin can call using this port. 
    // let port = "80";
    let url = format!("{}:{}", base_url, port);
    
    let listener = TcpListener::bind(url).unwrap();
    
    for (index, stream) in listener.incoming().enumerate() {
        let stream = stream.unwrap();
        println!("Connection #{} establised!", index);
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let ok = ("HTTP/1.1 200 OK", "hello.html");
    let not_found = ("HTTP/1.1 404 NOT FOUND", "404.html");
    
    let (status_line, filename) = if buffer.starts_with(get) {
        ok
    } else {
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
    
    // Above is the refactoring of the below code.
    // if buffer.starts_with(get) {
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let response = format!(
    //         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //         contents.len(),
    //         contents
    //     );
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let response = format!(
    //         "{}\r\nContent-Length: {}\r\n\r\n{}",
    //         status_line,
    //         contents.len(),
    //         contents,
    //     );
    // 
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }

}

