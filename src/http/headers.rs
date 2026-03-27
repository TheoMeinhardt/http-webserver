pub mod connection;
pub mod content_type;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct HeaderName(String);
pub struct HeaderValue(Vec<u8>);

impl HeaderName {
    pub fn new(name: &str) -> Self {
        let mut name_fmt = String::new();

        // Be true on init so the first letter is capitalized too.
        let mut prev_char_is_dash = true;
        for c in name.chars() {
            if prev_char_is_dash {
                name_fmt.push(c.to_ascii_uppercase());
            } else {
                name_fmt.push(c.to_ascii_lowercase());
            }

            if c == '-' {
                prev_char_is_dash = true;
            } else {
                prev_char_is_dash = false;
            }
        }

        Self(name_fmt)
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

pub struct Headers {
    inner: HashMap<HeaderName, Vec<HeaderValue>>,
}

impl Headers {
    pub fn new() -> Self {
        Headers {
            inner: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: HeaderName, value: Vec<HeaderValue>) {
        self.inner.insert(name, value);
    }

    pub fn set_single_value(&mut self, name: HeaderName, value: HeaderValue) {
        self.inner.insert(name, vec![value]);
    }

    pub fn get_all(&self) -> &HashMap<HeaderName, Vec<HeaderValue>> {
        &self.inner
    }

    pub fn encode(&self) -> String {
        let mut result = String::new();

        for (name, values) in &self.inner {
            // ignore header with no valuess
            if values.len() == 0 {
                continue;
            }

            result.push_str(name.0.as_str());
            result.push_str(": ");

            if values.len() == 1 {
                let value = values.first().unwrap();
                result.push_str(
                    String::from_utf8(value.0.clone())
                        .expect("Failed to parse bytes to characters!")
                        .as_str(),
                );
                result.push_str("\n");

                continue;
            }

            for value in values {
                result.push_str(
                    String::from_utf8(value.0.clone())
                        .expect("Failed to parse bytes to character!")
                        .as_str(),
                );
                result.push_str(",");
            }
            result.pop(); // remove trailing ','
            result.push_str("\n");
        }

        result
    }
}
