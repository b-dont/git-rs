use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};
use anyhow::Error;
use http::Request;

pub fn handle_request(mut stream: TcpStream) -> Result<(), Error> {
    let request = BufReader::new(&mut stream).lines().next();

    if let Some(req) = request {
    let mut request_line: Vec<&str> = request.split(" ").collect();

        let req = Request::builder()
            .method(request_line.collect::<Vec<&str>>()[0])
            .uri(request_line.collect::<Vec<&str>>()[1])
            .body(());
    };
    Ok(())
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