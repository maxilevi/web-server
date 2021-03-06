#![feature(str_split_as_str)]
use crate::http::method::Method;
use std::collections::HashMap;
use std::str::{Split, Utf8Error};

pub struct Request<'a> {
    method: Method,
    endpoint: &'a str,
    headers: HashMap<&'a str, &'a str>,
    body: &'a [u8],
}

impl<'a> Request<'a> {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn endpoint(&self) -> &str {
        self.endpoint
    }

    pub fn body(&self) -> &[u8] {
        self.body
    }

    pub fn body_utf8(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.body)
    }

    pub fn parse(request_bytes: &'a [u8]) -> Result<Request, &'static str> {
        let request_str =
        let mut lines = request_str.split("\r\n");

        let (method, endpoint) = Request::parse_method_and_endpoint(&mut lines)?;
        let headers = Request::parse_headers(&mut lines)?;

        let mut body = "";
        if headers.contains_key("Content-Length") {
            let length = headers["Content-Length"]
                .parse::<u32>()
                .ok()
                .ok_or("Invalid Content-Length header")?;
            // Si estuviéramos en Rust nightly podríamos usar lines.as_str() y evitar la allocation.
            // https://github.com/rust-lang/rust/issues/77998
            body = &request_str[(request_str.len() as u32 - length) as usize..];
        }

        Ok(Request {
            method,
            endpoint,
            headers,
            body,
        })
    }

    #[allow(dead_code)]
    pub fn headers(&self) -> &HashMap<&str, &str> {
        &self.headers
    }

    fn parse_method_and_endpoint(
        lines: &mut Split<'a, &'a str>,
    ) -> Result<(Method, &'a str), &'static str> {
        match lines.next() {
            Some(l) => {
                let parts = l.split(' ').collect::<Vec<&str>>();
                if parts.len() != 3 {
                    return Err("Malformed HTTP");
                }

                let method = Method::parse(parts[0])?;
                Ok((method, parts[1]))
            }
            None => Err("Malformed HTTP request"),
        }
    }

    fn parse_headers(
        lines: &mut Split<'a, &'a str>,
    ) -> Result<HashMap<&'a str, &'a str>, &'static str> {
        let mut headers = HashMap::new();

        loop {
            match lines.next() {
                Some(l) => {
                    let maybe_idx = l.find(':');
                    if maybe_idx.is_none() {
                        break;
                    }
                    let idx = maybe_idx.unwrap();
                    headers.insert(l[..idx].trim(), l[(idx + 1_usize)..].trim());
                }
                None => return Err("Malformed HTTP headers, none"),
            }
        }

        Ok(headers)
    }
}

impl ToString for Request<'_> {
    fn to_string(&self) -> String {
        let mut headers = self
            .headers
            .iter()
            .map(|x| format!("{}: {}", x.0.to_owned(), x.1.to_owned()))
            .collect::<Vec<String>>();
        headers.sort();
        format!(
            "{} {}\n{}\n{}",
            self.method.to_string(),
            self.endpoint,
            headers.join("\n"),
            self.body_utf8().unwrap_or("<invalid utf8 body>")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut headers = HashMap::new();
        headers.insert("Key", "Value");
        headers.insert("Test", "Header");
        headers.insert("Content-Length", "9");
        let request = Request::parse("POST /test HTTP/1.1\r\nContent-Length: 9\r\nKey: Value\r\nTest: Header\r\n\r\nTest body".as_bytes()).unwrap();
        assert_eq!(
            request.to_string(),
            Request {
                method: Method::Post,
                endpoint: "/test",
                headers,
                body: "Test body".as_bytes()
            }
            .to_string()
        );
    }
}
