use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, Error, BufRead};
use std::thread;
use serde_json::json;
use std::env;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut reader = BufReader::new(&stream);
    let mut headers = String::new();
    
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        
        // Stop when we reach an empty line (end of headers)
        if line == "\r\n" || line == "\n" {
            break;
        }
        headers.push_str(&line.trim());
    }

    let response = if headers.starts_with("GET /ping HTTP/1.1") {
        // Convert the headers to JSON
        let json_response = json!({
            "headers": headers.trim()
        });
        let body = json_response.to_string();
        format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}", body.len(), body)
    } else {
        "HTTP/1.1 404 Not Found\r\nContent-Length: 0".to_string()
    };

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}


fn main() {
    let name = "PING_LISTEN_PORT";
    let mut port: String = "".to_string();
    match env::var(name) {
        Ok(v) => port = v,
        Err(_e) => port = "8080".to_string()
    }
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Client handling error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
