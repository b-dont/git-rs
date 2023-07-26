use std::{
    net::TcpStream,
    io::{prelude::*, BufReader}
};

pub fn handle_request(mut stream: TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    return http_request
}
