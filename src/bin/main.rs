use echoserver::ThreadPool;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use chrono::prelude::*;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    let pool = ThreadPool::new(64);

    println!("Echoserver listening on port 3000...\n");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream).unwrap();
        })
    }

    println!("Shutting down");
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
	let mut reader = BufReader::new(&mut stream);
    let received: Vec<u8> = reader.fill_buf()?.to_vec();
    reader.consume(received.len());

    String::from_utf8(received)
        .map(|msg| println!("{}\n{} bytes\n{}", Local::now(),msg.len(),msg))
        .map_err(|err| {
			std::io::Error::new(
                std::io::ErrorKind::InvalidData,
				format!("Unable to parse string to UTF-8: {}", err)
            )
        }).unwrap();

    let response = format!("{}", "HTTP/1.1 200 OK\r\n\n");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    Ok(())
}
