use std::{
    env,
    net::TcpListener
};
use crate::connections::requests;

mod connections;

fn main() -> Result {
    let host = env::var("HOST");
    let port = env::var("PORT");

    let listener = TcpListener::bind(":".join(&[host, port]))?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        requests::handle_request(stream);
    }

    Ok(())
}
