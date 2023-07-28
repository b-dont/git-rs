use std::{
    net::TcpStream,
    io::{prelude::*, BufReader}
};

use http::Request;
use serde::ser;

// pub fn handle_request(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let http_request: Vec<_> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();
// }

pub fn build_req(mut stream: TcpStream) -> Request<()> {
    let buf_reader = BufReader::new(&mut stream);

    let raw_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let req = Request::builder()
        .method(raw_request[0].as_bytes())
        .body(())
        .unwrap();

    return req
}

pub fn serialize_req<T>(req: Request<()>) -> serde_json::Result<Request<Vec<u8>>> 
    where T: ser::Serialize,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::to_vec(&body)?;
    Ok(Request::from_parts(parts, body))
}

pub fn print_req(req: Request<()>) {
    println!("Request: {:#?}", req);
}
