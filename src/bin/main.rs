use echoserver::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    let pool = ThreadPool::new(4);

    println!("Listening on port 3000...\n");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();
    let req = std::str::from_utf8(&buffer).unwrap().replace("\u{0}", "");
    println!("{} bytes", req.len());
    println!("{}\n\n", req);

    let response = format!("{}", "HTTP/1.1 200 OK\r\n\n");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
