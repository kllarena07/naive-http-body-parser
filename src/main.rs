use native_tls::TlsConnector;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = "jsonplaceholder.typicode.com";
    let port = 443; // HTTPS default port
    let path = "/todos/1";
    let connector = TlsConnector::new().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let stream = TcpStream::connect(format!("{}:{}", host, port))?;
    let mut stream = connector
        .connect(host, stream)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let request = format!(
        "GET {} HTTP/1.1\r\n\
         Host: {}\r\n\
         Connection: close\r\n\
         \r\n",
        path, host
    );

    stream.write_all(request.as_bytes())?;

    let mut response_buffer = Vec::new();
    stream.read_to_end(&mut response_buffer)?;

    let response_str = String::from_utf8_lossy(&response_buffer);
    println!("{}", response_str);

    Ok(())
}
