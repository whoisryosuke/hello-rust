use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Parse stream
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // BufReader lets us read/write to stream (which we also make mutable)
    let buf_reader = BufReader::new(&mut stream);
    //
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    // Should print out
    // Request: [
    //     "GET / HTTP/1.1",
    //     "Host: 127.0.0.1:7878",
    //     "Connection: keep-alive",
    //     "Cache-Control: max-age=0",
    //     "sec-ch-ua: \"Chromium\";v=\"106\", \"Google Chrome\";v=\"106\", \"Not;A=Brand\";v=\"99\"",
    //     "sec-ch-ua-mobile: ?0",
    //     "sec-ch-ua-platform: \"Windows\"",
    //     "Upgrade-Insecure-Requests: 1",
    //     "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36",
    //     "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9",
    //     "Sec-Fetch-Site: none",
    //     "Sec-Fetch-Mode: navigate",
    //     "Sec-Fetch-User: ?1",
    //     "Sec-Fetch-Dest: document",
    //     "Accept-Encoding: gzip, deflate, br",
    //     "Accept-Language: en-US,en;q=0.9,ja;q=0.8,pt;q=0.7",
    // ]

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
