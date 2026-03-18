use http_webserver::server::Server;

fn main() {
    let server = Server::new("127.0.0.1", 1234);
    server.listen();
}
