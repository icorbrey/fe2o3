use regex::Regex;
use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    str,
};

/// Represents a request made to a TCPListener.
pub struct Request {
    // method: String,
    // http_version: String,
    /// The path requested.
    path: String,
}

impl Request {
    /// Converts an incoming TCP request into a Request object.
    ///
    /// ### Arguments
    ///
    /// * `stream` - The incoming TCP request to convert.
    ///
    /// ### Example
    ///
    /// ```
    /// mod webjob;
    /// use webjob::Request;
    /// use std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:80");
    /// let request = Request::from(&stream);
    /// ```
    pub fn from(mut stream: &TcpStream) -> Request {
        let text = Request::get_text(&mut stream);
        let (_method, path, _http_version) = Request::parse_request(text);
        Request {
            // method,
            path,
            // http_version,
        }
    }

    /// Returns the text sent in the incoming stream.
    ///
    /// ### Arguments
    ///
    /// * `stream` - The incoming TCP stream to get the text of.
    fn get_text(mut stream: &TcpStream) -> String {
        let mut buffer = [0; 512];
        stream
            .read(&mut buffer)
            .expect("Could not read from request stream.");
        String::from(str::from_utf8(&buffer).expect("Could not convert request to UTF-8."))
    }

    /// Returns the request's method, path, and HTTP version.
    ///
    /// ### Arguments
    ///
    /// * `text` - The incoming TCP request to parse.
    fn parse_request(text: String) -> (String, String, String) {
        let mut r = text.split_whitespace().take(3);
        let mut result = (String::new(), String::new(), String::new());

        if let Some(string) = r.nth(0) {
            result.0 = String::from(string);
        }
        if let Some(string) = r.nth(1) {
            result.1 = String::from(string);
        }
        if let Some(string) = r.nth(2) {
            result.2 = String::from(string);
        }

        result
    }
}

/// Represents a response to be sent to a TcpStream.
pub struct Response {
    status: String,
    payload: String,
}

impl Response {
    /// Returns a response to the given request based on the given paths.
    ///
    /// ### Arguments
    ///
    /// * `request` - The request to respond to.
    /// * `paths` - A list of regex-matched paths and their respective filenames.
    ///
    /// ### Example
    ///
    /// ```
    /// mod webjob;
    /// use webjob::Request;
    /// use std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:80");
    /// let request = Request::from(&stream);
    /// let response = Response::new(request, vec![
    ///     ("/", "wwwroot/index.html"),
    ///     (".*", "wwwroot/not_found.html"),
    /// ]);
    /// ```
    pub fn new(request: Request, paths: Vec<(&str, &str)>) -> Response {
        let mut status = String::new();
        let mut payload = String::new();

        for (expression, path) in paths {
            if Response::matches_path(expression, request.path.as_str()) {
                status = String::from("200 OK");
                payload = Response::get_file(path);
                break;
            }
        }

        Response { status, payload }
    }

    /// Returns whether the given path matches the expression.
    ///
    /// ### Arguments
    ///
    /// * `expression` - The regex-matched path to match.
    /// * `path` - The path to check against the expression.
    fn matches_path(expression: &str, path: &str) -> bool {
        let regex = Regex::new(expression)
            .expect(format!("Could not create regex object from {}", expression).as_str());
        regex.is_match(path)
    }

    /// Returns The contents of the given file.
    ///
    /// ### Arguments
    ///
    /// * `path` - The path to read text from.
    fn get_file(path: &str) -> String {
        fs::read_to_string(path).expect(&format!("Could not read file at {}", path))
    }

    /// Sends the response to the given TcpStream.
    ///
    /// ### Arguments
    ///
    /// * `stream` - The outbound stream to write to.
    ///
    /// ### Example
    ///
    /// ```
    /// mod webjob;
    /// use webjob::Request;
    /// use std::net::TcpStream;
    ///
    /// let stream = TcpStream::connect("127.0.0.1:80");
    /// let request = Request::from(&stream);
    /// let response = Response::new(request, vec![
    ///     ("/", "wwwroot/index.html"),
    ///     (".*", "wwwroot/not_found.html"),
    /// ]);
    /// response.send(&stream);
    /// ```
    pub fn send(&self, stream: &mut TcpStream) {
        let text = format!("HTTP/1.1 {}\r\n\r\n{}", self.status, self.payload);
        stream
            .write(text.as_bytes())
            .expect("Could not write to response stream");
        stream.flush().expect("Could not flush response stream");
    }
}
