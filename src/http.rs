use std::fmt;
use std::io::prelude::*;
use std::net::TcpStream;
use url::Url;
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

pub struct HttpRequest {
    method: Method,
    path: String,
    host: String,
    accept: String,
    connection: String,
    url: Url,
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buff = vec![];
        buff.extend_from_slice(self.method.to_string().as_bytes());
        buff.extend_from_slice(" ".as_bytes());
        buff.extend_from_slice(self.path.as_bytes());
        buff.extend_from_slice(" HTTP/1.1\r\n".as_bytes());
        buff.extend_from_slice("HOST: ".as_bytes());
        buff.extend_from_slice(self.host.as_bytes());
        buff.extend_from_slice("\r\n".as_bytes());
        buff.extend_from_slice("Accept: ".as_bytes());
        buff.extend_from_slice(self.accept.as_bytes());
        buff.extend_from_slice("\r\n".as_bytes());
        buff.extend_from_slice("Connection: ".as_bytes());
        buff.extend_from_slice(self.connection.as_bytes());
        buff.extend_from_slice("\r\n\r\n".as_bytes());
        write!(f, "{}", std::str::from_utf8(&buff).expect("The buffer was formed from valid utf-8 strings, and this should be able to be converted back"))
    }
}

impl HttpRequest {
    pub fn new(url: &str) -> anyhow::Result<Self> {
        let url = Url::parse(url)?;
        Ok(HttpRequest {
            method: Method::GET,
            path: url.path().to_string(),
            host: url
                .host_str()
                .ok_or_else(|| anyhow::format_err!("The URl does not have a valid hose"))?
                .to_string(),
            accept: "*".to_string(),
            connection: "keep-alive".to_string(),
            url: url,
        })
    }

    pub fn run(&self) -> anyhow::Result<(usize, Vec<u8>)> {
        let host = self.url.host_str().unwrap();
        let mut addr = host.to_string();
        addr.push_str(":80");
        let mut stream = TcpStream::connect(addr)?;
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
