use std::io;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
    // multiple variable ?
    let (host, port) = ("0.0.0.0", "8080");

    // string join with coron":".
    let conn = [host, port].connect(":");

    // listen "0.0.0.0:8080"
    let server = TcpListener::bind(conn).unwrap();

    // print server's local address.
    println!("Server listening: {}", server.local_addr().unwrap());

    // wating client.
    for stream in server.incoming() {
        match stream {
            // if accepted
            Ok(_s) => {
                // using thread because blocking call.
                thread::spawn(move || {
                    print!("accepted: ");
                    handler(&_s)
                });
            }
            // if EWOULDBLOCK, continue (?)
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            // maybe, think rasing error.
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }
}

// client handler function.
fn handler(mut client: &TcpStream) {
    // for recv buffer, size is 1 KiB.
    let mut recv = [0u8; 1024];

    // read from client stream.
    if let &Err(ref e) = &client.read(&mut recv) {
        panic!("error during receive a line: {}", e);
    }

    // body
    let body = "Hello World";
    
    // main response
    let mut response = String::from(concat!(
        "HTTP/1.1 200 OK\r\n",
        "Content-Type: text/html; charset=UTF-8\r\n",
        "Content-Length: "
    ));

    // push, puuuuush ;)
    response.push_str(&mut body.len().to_string());
    response.push_str("\r\n");
    response.push_str("Connection: Close\r\n\r\n");
    response.push_str(body);
    
    // borrow :)
    &client.write(response.as_bytes()).unwrap();
}