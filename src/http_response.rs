use std::fmt;
use crate::content_types::ContentTypes;
use crate::server::{SERVER_HTTP_VERSION, SERVER_NAME};

#[derive(Debug)]
pub struct HttpResponse {
    pub response_code: u16,
    pub response_message: String,
    pub body: String,
    pub content_type: ContentTypes,
}

// {} = placeholder wie String.format(%s)
impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} \r\n\
             Server: {} \r\n\
             content-length: {} \r\n\
             Content-Type: {} \r\n\
             \r\n\
             {}",
            SERVER_HTTP_VERSION,
            self.response_code,
            self.response_message,
            SERVER_NAME,
            self.body.as_bytes().len(),
            self.content_type,
            self.body
        )
    }
}