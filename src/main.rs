use std::io::{BufRead, Write};
use std::net::TcpListener;

fn main() {
    // Using TcpListener, we can listen for Transmission Control Protocol (TCP) connections at the address 127.0.0.1:9999
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    // listener.incoming() creates an iterator over incoming TCP connections.
    // Each item yielded by this iterator is a Result<TcpStream, io::Error>, where TcpStream represents
    // a connection and io::Error represents a potential error.
    //
    // .flatten() transform the iterator of Result<TcpStream, io::Error> into an iterator of TcpStream, ignoring any errors
    for mut stream in listener.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&mut stream);

        loop {
            let mut l = String::new();
            rdr.read_line(&mut l).unwrap();

            // HTTP messages contain a blank line indicating all meta-information for the request has been sent
            // https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages#http_requests
            if l.trim().is_empty() {
                break;
            }

            print!("{l}");
        }

        // Send an HTTP response back to the TCP stream in the expected format
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages#http_responses
        stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello!").unwrap();
    }
}
