pub struct Response {
    request: String,
    status_line: String,
    contents: String,
    length: usize
}

impl Response {
    fn build_response(request: String, status_line: String, contents: String, length: usize) -> Response {
        Response {
            request,
            status_line,
            contents,
            length
        }
    }
}
