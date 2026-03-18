use std::net::TcpStream;

struct Handler {
    route: String,
    method: String,
    callback: fn(),
}
