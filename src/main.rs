use std::net::TcpListener;
use anyhow::Error;
use crate::connections::requests;

use dotenv::{
    dotenv,
    var
};

mod connections;

fn main() -> Result<(), Error> {
    // Env variables stored in .env at project root
    dotenv().ok();
    let host = "HOST";
    let port = "PORT";
    
    let listen_location = vec![
        var(host)?,
        var(port)?,
    ];

    // Listener object bound to port defined in .env
    let listener = TcpListener::bind(listen_location.join(":"))?;

    // Main program loop handling incoming TCP reqeusts
    for stream in listener.incoming() {
        requests::handle_request(stream?);
    }
    Ok(())
}
