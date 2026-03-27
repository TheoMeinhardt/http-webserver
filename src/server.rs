use crate::http::{Body, HeaderName, HeaderValue, Headers, Response, content_type::ContentType};
use httpstatus::StatusCode;
use std::net::TcpListener;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    // Creates and binds a new TCP socket server on a specified address and port.
    pub fn new(addr: &str, port: u32) -> Self {
        let listener = TcpListener::bind(format!("{addr}:{port}"))
            .expect("Failed to start server! Is your port already used?");

        println!(
            "Server listening on port {}",
            listener.local_addr().unwrap()
        );

        Server { listener }
    }

    // Starts listening for incoming connections.
    pub fn listen(self) {
        for stream in self.listener.incoming() {
            // println!("Handeling {}", stream.unwrap().local_addr().unwrap().ip());

            let mut headers = Headers::new();
            headers.set_single_value(HeaderName::new("Connection"), HeaderValue::from("close"));

            let res = Response::new(
                String::from("HTTP/1.1"),
                StatusCode::Ok,
                headers,
                Body::new(
                    ContentType::TextHtml,
                    String::from("<!DOCTYPE html><html><body><h3>Hello HTML!</h3></body></html>"),
                ),
            );
            res.send(&mut stream.unwrap());
        }
    }
}
