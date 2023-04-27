use std::{
    fs,
    fmt,
    io::{prelude::*, BufReader},
    net::TcpStream,
};
use anyhow::Error;

#[derive(Debug)]
pub enum Request {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn handle_request(mut stream: TcpStream) -> Result<(), Error> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next();

    match request_line {
        Request::Get => ,
        _ => ,
    }
    
    // if request_line == "GET / HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("templates/index.html").unwrap();
    //     let length = contents.len();
    
    //     let response =
    //         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    //     stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("templates/404.html").unwrap();
    //     let length = contents.len();

    //     let response = format!(
    //         "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    //     );
    //     stream.write_all(response.as_bytes()).unwrap();
    // }
}