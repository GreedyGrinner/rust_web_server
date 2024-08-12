
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use crate::content_types::ContentTypes;
use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;

pub const SERVER_NAME: &str = "sprechaserva";
pub const SERVER_HTTP_VERSION: &str = "HTTP/1.1";
pub const SERVER_IP: &str = "127.0.0.1:4200";

pub fn serve() {
    // listens to TCP messages to this server
    let tcp_listener = TcpListener::bind(SERVER_IP).unwrap();

    // for each incoming stream do something
    for stream in tcp_listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        // new thread for each connection
        thread::spawn(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    // read stream with buffered reader
    let buf_reader = BufReader::new(&mut stream);

    let http_request_string: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let http_request_string_split: Vec<&str>= http_request_string.first().unwrap().split(" ").collect();
    let method = String::from(http_request_string_split[0]);
    let path = String::from(http_request_string_split[1]);
    let http_version = String::from(http_request_string_split[2]);
    let http_request = HttpRequest { method, path, http_version, body: "greg".to_string() };
    println!("{http_request:#?}");

    if !http_request.http_version.eq(SERVER_HTTP_VERSION) {
        let response_code:u16 = 404;
        let response_message = String::from("ERROR");
        let body = String::from("error\r\n");
        let content_type = ContentTypes::STRING;
        let http_response = HttpResponse { response_code, response_message, body, content_type };
        println!("{}", http_response);
        stream.write_all(http_response.to_string().as_bytes()).unwrap();
    } else {
        let response_code:u16 = 200;
        let response_message = String::from("OK");
        let body = String::from("<html><bod><p>Hello World!</p></body></html>\r\n");
        let content_type = ContentTypes::HTML;
        let http_response = HttpResponse { response_code, response_message, body, content_type };
        println!("{}", http_response);
        stream.write_all(http_response.to_string().as_bytes()).unwrap();
    }

}