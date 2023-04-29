use std::{
    fs,
    fmt,
    io::{prelude::*, BufReader},
    net::TcpStream,
};
use anyhow::Error;
use http::Response;

pub fn handle_request(mut stream: TcpStream) -> Result<(), Error> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next();

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
    Ok(())
}