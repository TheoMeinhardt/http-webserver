use crate::http::content_type::ContentType;

pub struct Body {
    content_type: Option<ContentType>,
    content: String,
}

impl Body {
    pub fn new(content_type: ContentType, content: String) -> Self {
        Self {
            content_type: Some(content_type),
            content,
        }
    }

    pub fn empty() -> Self {
        Self {
            content_type: None,
            content: String::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.content_type {
            Some(_) => false,
            None => true,
        }
    }

    pub fn get_content_type(&self) -> &ContentType {
        &self.content_type.as_ref().unwrap()
    }

    pub fn encode(&self) -> String {
        let mut result = String::from("\r\n");
        result.push_str(&self.content);

        result
    }

    pub fn len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        return self.content.as_bytes().len();
    }
}
