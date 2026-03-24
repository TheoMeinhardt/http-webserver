use std::{
    io::{BufWriter, Write},
    net::{Shutdown, TcpStream},
};

use crate::http::Headers;

pub struct Response {
    http_version: String,
    status_code: u16,
    reason_phrase: String,
    headers: Headers,
}

impl Response {
    pub fn new(
        http_version: String,
        status_code: u16,
        reason_phrase: String,
        headers: Headers,
    ) -> Self {
        Response {
            http_version,
            status_code,
            reason_phrase,
            headers,
        }
    }

    pub fn send(self, stream: &mut TcpStream) -> () {
        let mut writer = BufWriter::new(stream);

        // write HTTP status line to BufWriter,
        // i.e. HTTP/1.1 200 Ok
        writer
            .write_all(
                format!(
                    "{} {} {}\r\n",
                    self.http_version, self.status_code, self.reason_phrase
                )
                .as_bytes(),
            )
            .expect("Failed to write to Stream!");

        // write headers to BufWriter,
        // i.e. connection: close
        writer
            .write_all(self.headers.encode().as_bytes())
            .expect("Failed to write to Stream!");

        writer.flush().expect("Failed to flush BufWriter!");

        writer
            .get_mut()
            .shutdown(Shutdown::Both)
            .expect("Failed to shutdown Stream!");
    }
}
