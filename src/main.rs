use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = hello_rust::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // 👉 Spawn a thread for every request
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Handle each route/URL path and request type (GET/POST/etc)
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "index.html"),
    };

    // Load the HTML or content
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    // Create the response
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Send response to user
    stream.write_all(response.as_bytes()).unwrap();
}
