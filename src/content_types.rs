use std::fmt;

#[derive(Debug)]
pub enum ContentTypes {
    HTML, STRING, JSON
}

// rust toString
impl fmt::Display for ContentTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content_type_str = match self {
            ContentTypes::HTML => "text/html",
            ContentTypes::STRING => "plain/string",
            ContentTypes::JSON => "application/json",
        };
        write!(f, "{}", content_type_str)
    }
}