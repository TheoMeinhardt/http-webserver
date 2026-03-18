use std::{
    io::{BufReader, BufWriter},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

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
            println!("Handeling {}", stream.unwrap().local_addr().unwrap().ip());
        }
    }
}
