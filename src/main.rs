use std::net::TcpListener;

fn main() {
    // Listen for TCP connections at the following URL/port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // This loops over all the "streams" or open connections with client

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
