use crate::http::headers::{Header, HeaderName, HeaderValue};

pub enum Connection {
    KeepAlive,
    Close,
}

pub struct ConnectionHeader(pub Connection);

impl Connection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Connection::KeepAlive => "keep-alive",
            Connection::Close => "close",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "keep-alive" => Some(Connection::KeepAlive),
            "close" => Some(Connection::Close),
            _ => None,
        }
    }
}

impl Header for ConnectionHeader {
    fn name() -> HeaderName {
        HeaderName::new("connection")
    }

    fn encode(&self) -> Vec<HeaderValue> {
        vec![HeaderValue::from(self.0.as_str())]
    }

    fn decode(values: &[HeaderValue]) -> Option<Self> {
        let value = values.first()?;
        let s = std::str::from_utf8(&value.0).ok()?;
        let con = Connection::parse(s)?;

        Some(ConnectionHeader(con))
    }
}
