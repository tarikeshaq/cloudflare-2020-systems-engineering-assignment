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
            connection: "close",
        })
    }

    pub fn run(&self) -> anyhow::Result<(usize, Vec<u8>, std::time::Duration)> {
        let mut addr = self.host.to_string();
        addr.push_str(":80");
        let mut stream = TcpStream::connect(&addr)?;
        stream.write(self.to_string().as_bytes())?;
        let mut buff = vec![0u8; 4096];
        let instant = std::time::Instant::now();
        // We read till the socket is closed on the server side
        // this is okay since we indicate in our request that we
        // would like to close the connection once the request is serviced
        // NOTE: Big problem here is that we are **NOT** streaming the recieved data
        // so in the case we recieve more data than our heap can handle, we will
        // crash and burn. Is it possible? Yes, is it probable? well no

        // A better approach if I had the time would be to use a BufReader and return that
        // then, when we print or manupilate the response, we read as we go

        // An even better approach would be to read the headers first (reading till we find a `\r\n\r\n`)
        // investigate the `Content-length` or the `Transfer-Encoding` and appropriately decide how much to read from the response
        // buuut a clean way to do that would involve desearializing the headers into Rust types, supporting content-length and chunking
        // and add a few more features and we'd have a full Http library :)
        let amount_read = stream.read_to_end(&mut buff)?;
        Ok((amount_read, buff, instant.elapsed()))
    }
}
