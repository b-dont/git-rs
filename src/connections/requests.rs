use std::{
    net::TcpStream,
    io::{prelude::*, BufReader}
};

use http::Request;

pub fn handle_request(mut stream: TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    return http_request
}

pub fn build_request(mut stream: TcpStream) -> Request<()> {
    let buf_reader = Some(BufReader::new(&mut stream));

    if let Some(req_line) = buf_reader {
        req_line = buf_reader.lines()?.next()
    }

    let mut req = Request::builder()
        .uri()
        .header();

    return req
}
