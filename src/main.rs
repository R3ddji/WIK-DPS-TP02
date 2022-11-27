use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;
use std::io::BufReader;

fn main() {
    let ipport = match env::var("PING_LISTEN_PORT") {
        Ok(keyenv) => format!("0.0.0.0:{keyenv}"),
        Err(_e) => format!("0.0.0.0:8888"),
    };

    let listener = TcpListener::bind(ipport).unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request[0] == "GET /ping HTTP/1.1" {

        let mut valfi = String::new();
        let mut first = true;

        for db in &http_request {

            match db.split_once(':') {
                Some((key, value)) => {
                    if first {
                        valfi = valfi + "\"" +key + "\"" + " : " + "\"" + value + "\"";
                        first = false;
                    } else {
                        valfi = valfi + "," + "\"" +key + "\"" + " : " + "\"" + value + "\"";
                    }
                    
                }
                None => {
                    println!("expected a key-value pair");
                }
            }
        }

        let status_line = "HTTP/1.1 200 OK";
        let response = format!("{status_line}\r\nContent-Type: application/json;\r\n\r\n{{{valfi}}}");    

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let response = format!("{status_line}\r\n\r\n");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
