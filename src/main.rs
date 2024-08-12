mod server;
pub mod http_response;
pub mod http_request;
pub mod content_types;

fn main() {
    server::serve();
}
