use std::fmt;
use std::io::prelude::*;
use std::net::TcpStream;
enum Method {
    GET,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Method::GET => "GET",
        };
        write!(f, "{}", s)
    }
}

pub struct HttpRequest<'a> {
    method: Method,
    path: &'a str,
    host: &'a str,
    accept: &'a str,
    connection: &'a str,
}

impl<'a> fmt::Display for HttpRequest<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} HTTP/1.1\r\n\
        HOST: {}\r\n\
        Accept: {}\r\n\
        Connection: {}\r\n\r\n",
            self.method, self.path, self.host, self.accept, self.connection
        )
    }
}

impl<'a> HttpRequest<'a> {
    pub fn new(url: &'a url::Url) -> anyhow::Result<Self> {
        Ok(HttpRequest {
            method: Method::GET,
            path: url.path(),
            host: url
                .host_str()
                .ok_or_else(|| anyhow::format_err!("The URl does not have a valid hose"))?,
            accept: "*",
            connection: "keep-alive",
        })
    }

    pub fn run(&self) -> anyhow::Result<(usize, Vec<u8>)> {
        let mut addr = self.host.to_string();
        addr.push_str(":80");
        let mut stream = TcpStream::connect(&addr)?;
        stream.write(self.to_string().as_bytes())?;
        // To make things uhhh "easier", we only care about the first 2KB
        // of response data if there is more.
        // There are ways to grab the full response, but that would require digging into
        // the headers, pulling out the Content-Length/Transfer-Encoding
        let mut buff = vec![0u8; 2096];
        let amount_read = stream.read(&mut buff)?;
        Ok((amount_read, buff))
    }
}
