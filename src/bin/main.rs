// extern crate threadpool; // legacy
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

const HTTP_DEFAULT_PORT: &str = "127.0.0.1:8080";
const N_WORKER_THREADS: usize = 2;

// code will panic! --> handle unwraps at some point
fn main() {
    // needs config file

    // bind to socket
    let listener = TcpListener::bind(HTTP_DEFAULT_PORT).unwrap();
    println!(
        "Server listening on {}:{}",
        listener.local_addr().unwrap().ip(),
        listener.local_addr().unwrap().port()
    );

    // create threadpool
    let threadpool = ThreadPool::new(N_WORKER_THREADS);

    // accept new connections
    for client in listener.incoming() {
        let client = client.unwrap();

        threadpool.execute(|| handle_connection(client));
    }
}

fn handle_connection(mut client: TcpStream) {
    println!("handling connection!");
}