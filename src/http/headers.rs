pub mod connection;

pub struct HeaderName(String);
pub struct HeaderValue(Vec<u8>);

impl HeaderName {
    pub fn new(name: &str) -> Self {
        Self(name.to_ascii_lowercase())
    }
}

impl From<&str> for HeaderValue {
    fn from(s: &str) -> Self {
        Self(s.as_bytes().to_vec())
    }
}

pub trait Header: Sized {
    fn name() -> HeaderName;
    fn encode(&self) -> Vec<HeaderValue>;
    fn decode(values: &[HeaderValue]) -> Option<Self>;
}
