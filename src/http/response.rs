use std::{
    io::{BufWriter, Write},
    net::{Shutdown, TcpStream},
    time::SystemTime,
};

use crate::http::{HeaderName, HeaderValue, Headers};

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

    pub fn send(mut self, stream: &mut TcpStream) -> () {
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

        // set a few default headers and
        // the date header
        self.set_default_headers();
        self.set_date_header();

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

    fn set_default_headers(&mut self) {
        self.headers.set_single_value(
            HeaderName::new("X-Content-Type-Options"),
            HeaderValue::from("nosniff"),
        );
        self.headers.set_single_value(
            HeaderName::new("X-Frame-Options"),
            HeaderValue::from("deny"),
        );
        self.headers
            .set_single_value(HeaderName::new("X-XSS-Protection"), HeaderValue::from("0"));
        self.headers.set_single_value(
            HeaderName::new("Referrer-Policy"),
            HeaderValue::from("no-referrer-when-downgrade"),
        );
        self.headers.set_single_value(
            HeaderName::new("Content-Security-Policy"),
            HeaderValue::from("default-src 'self'"),
        );
    }

    fn set_date_header(&mut self) {
        self.headers.set_single_value(
            HeaderName::new("date"),
            HeaderValue::from(httpdate::fmt_http_date(SystemTime::now()).as_str()),
        );
    }
}
