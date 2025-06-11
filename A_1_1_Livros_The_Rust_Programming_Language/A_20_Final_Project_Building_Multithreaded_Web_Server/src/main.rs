/// HTTP is a text-based protocol, and a request takes this format:
///     Method Request-URI HTTP-Version CRLF
///     headers CRLF
///     message-body
///
/// Responses have the following format:
///     HTTP-Version Status-Code Reason-Phrase CRLF
///     headers CRLF
///     message-body
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use A_20_Final_Project_Building_Multithreaded_Web_Server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() { // use listener.incoming().take(2) to simulate graceful shutdown
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });

        println!("Shutting down.");
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        // let http_request: Vec<_> = buf_reader
        //     .lines()
        //     .map(|result| result.unwrap())
        //     .take_while(|line| !line.is_empty())
        //     .collect();

        // println!("Request {:#?}", http_request);

        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let (status_line, filename) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(15));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };

        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();
        let response = format!(
            "{status_line}\r\n\
                Content-Length: {length}\r\n\r\n\
                {contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}
