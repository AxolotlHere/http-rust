use crate::pool::ThreadPool;
use std::fs;
use std::io::{BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
mod pool;

fn main() {
    let listen_ = TcpListener::bind("192.168.0.108:7878").unwrap(); //Result<> return
    //Finite thread pool
    let thread_pool = ThreadPool::new(5);
    for i in listen_.incoming() {
        let i = i.unwrap();

        thread_pool.execute(|| handle_conn(i));
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let req: String = buf_reader.lines().next().unwrap().unwrap();
    println!("{req:#?}");

    //sending response
    if req == "GET / HTTP/1.1" {
        let resp_status: &str = "HTTP/1.1 200 OK";
        let html_content: String = fs::read_to_string("index.html").unwrap();
        let content_len: usize = html_content.len();
        stream
            .write_all(
                format!("{resp_status}\r\nContent-Length:{content_len}\r\n\r\n{html_content}")
                    .as_bytes(),
            )
            .unwrap();
    } else if req == "GET /sleep HTTP/1.1" {
        thread::sleep(Duration::from_millis(500));
        let resp_status: &str = "HTTP/1.1 200 OK";
        let html_content: String = fs::read_to_string("index.html").unwrap();
        let content_len: usize = html_content.len();
        stream
            .write_all(
                format!("{resp_status}\r\n\r\nContent-Length:{content_len}\r\n\r\n{html_content}")
                    .as_bytes(),
            )
            .unwrap();
    } else {
        let resp_status: &str = "HTTP/1.1 404 NOT FOUND";
        let html_content: String = "404 Page not found".to_string();
        let content_len: usize = html_content.len();
        stream
            .write_all(
                format!("{resp_status}\r\nContent-Length:{content_len}\r\n\r\n{html_content}")
                    .as_bytes(),
            )
            .unwrap();
    }
}
