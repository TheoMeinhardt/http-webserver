use console::Style;
use httpstatus::StatusCode;
use std::{
    io::{BufWriter, Write},
    net::{Shutdown, TcpStream},
    time::SystemTime,
};

use crate::http::{Body, HeaderName, HeaderValue, Headers, content_type::ContentType};

pub struct Response {
    http_version: String,
    status: StatusCode,
    headers: Headers,
    body: Body,
}

impl Response {
    pub fn new(http_version: String, status: StatusCode, headers: Headers, body: Body) -> Self {
        Response {
            http_version,
            status,
            headers,
            body,
        }
    }

    pub fn send(mut self, stream: &mut TcpStream) -> () {
        let mut writer = BufWriter::new(stream);

        // write HTTP status line to BufWriter,
        // i.e. HTTP/1.1 200 Ok
        writer
            .write_all(format!("{} {}\r\n", self.http_version, self.status.to_string()).as_bytes())
            .expect("Failed to write to Stream!");

        // set a few default headers,
        // the date header and the content-type header,
        // if body is not empty.
        self.set_default_headers();
        self.set_date_header();
        if !self.body.is_empty() {
            self.set_content_type_header(self.body.get_content_type().to_string());
        }

        // write headers to BufWriter,
        // i.e. connection: close
        writer
            .write_all(self.headers.encode().as_bytes())
            .expect("Failed to write Headers to Stream!");

        // write body to BufWriter
        // `Body::encode` automatically takes care of the empty line
        writer
            .write_all(self.body.encode().as_bytes())
            .expect("Failed to write body to Stream!");

        writer.flush().expect("Failed to flush BufWriter!");

        self.log(&writer.get_ref().peer_addr().unwrap().to_string());

        writer
            .get_mut()
            .shutdown(Shutdown::Both)
            .expect("Failed to shutdown Stream!");
    }

    fn log(&self, target: &String) {
        let col = match &self.status.as_u16() {
            100..=199 => Style::new().fg(console::Color::White),
            200..=299 => Style::new()
                .bg(console::Color::TrueColor(0, 168, 107))
                .fg(console::Color::TrueColor(0, 0, 0)),
            300..=399 => Style::new()
                .bg(console::Color::TrueColor(220, 220, 220))
                .fg(console::Color::Black),
            400..=499 => Style::new()
                .bg(console::Color::White)
                .fg(console::Color::Black),
            500..=599 => Style::new().bg(console::Color::TrueColor(220, 29, 19)),
            _ => Style::new().white(),
        };

        println!(
            "sent {} to {}",
            col.apply_to(self.status.to_string()),
            target
        );
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

    fn set_content_type_header(&mut self, content_type: String) {
        self.headers.set_single_value(
            HeaderName::new("content-type"),
            HeaderValue::from(content_type.as_str()),
        );
    }
}
