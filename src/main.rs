use native_tls::TlsConnector;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

fn get_content_length(response_str: &str) -> usize {
    let i = response_str.find("Content-Length:").unwrap();
    let j = &response_str[i + 16..].find("\r\n").unwrap();
    let content_length = &response_str[i + 16..i + 16 + j].parse::<usize>().unwrap();
    content_length.to_owned()
}

fn get_request_body(response_str: &str) -> String {
    let content_length = get_content_length(response_str);
    let k = response_str.find("\r\n\r\n").unwrap();
    // add 4 bytes to skip "\r\n\r\n"
    response_str[k + 4..k + 4 + content_length].to_owned()
}

fn parse_json(json_str: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    // Isolate the keys and values
    let trimmed = &json_str[1..json_str.len() - 1];
    let replaced = trimmed
        .replace("\n", "")
        .replace("\r", "")
        .replace("\"", "");
    let split: Vec<&str> = replaced.split(",").collect();

    // Extract keys and values and add to hashmap
    let mut map = HashMap::new();
    for pair in split {
        let parts: Vec<&str> = pair.split(": ").collect();
        if parts.len() == 2 {
            map.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
            continue;
        }

        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid JSON format",
        )) as Box<dyn std::error::Error>);
    }

    Ok(map)
}

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

    let response_str = String::from_utf8(response_buffer)?;
    let request_body = get_request_body(&response_str);

    let json = parse_json(&request_body).ok();
    println!("{:?}", json);

    Ok(())
}
