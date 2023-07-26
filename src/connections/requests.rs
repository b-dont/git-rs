use std::net::TcpStream;

use http::Request;
use serde::ser;

pub fn handle_request<T>(req: TcpStream) -> serde_json::Result<Request<Vec<u8>>>
    where T: ser::Serialize,
{
    let (parts, body) = req.();
    let body = serde_json::to_vec(&body)?;
    Ok(Request::from_parts(parts, body))
}
