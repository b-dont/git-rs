use std::net::TcpListener;
use crate::connections::handlers;

mod connections;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handlers::handle_connection(stream);
    }
}
