use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

extern crate http_server;
use http_server::ThreadPool;

const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1:8080";


fn main() -> std::io::Result<()> {
    // startup
    // config parser
    // command thread

    // bind to port
    let listener = TcpListener::bind(DEFAULT_SERVER_ADDRESS)?;
    println!(
        "Listening on {}:{}",
        listener.local_addr().unwrap().ip(),
        listener.local_addr().unwrap().port()
    );

    // create threadpool (workers for single connections)
    let pool = ThreadPool::new(4).unwrap();

    // accept connections (why do you need 2 again?)
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // receive message from client
    stream.read(&mut buffer).unwrap();
    // println!("Received: {}", String::from_utf8_lossy(&buffer));

    // respond to client
    let (status_line, filename) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_millis(3000));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write_all(response.as_bytes()).unwrap();
    // println!("Wrote:\n{}", response);

}
