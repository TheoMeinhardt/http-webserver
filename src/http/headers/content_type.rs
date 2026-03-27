use std::fmt;

pub enum ContentType {
    TextPlain,
    TextHtml,
    TextCss,
    ApplicationJson,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::TextPlain => write!(f, "text/plain"),
            ContentType::TextHtml => write!(f, "text/html"),
            ContentType::TextCss => write!(f, "text/css"),
            ContentType::ApplicationJson => write!(f, "application/json"),
        }
    }
}
