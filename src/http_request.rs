#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub body: String
}