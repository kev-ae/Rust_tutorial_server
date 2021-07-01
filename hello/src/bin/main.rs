use hello::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    let mut sites = vec!["hello"];
    sites.sort();
    let sites = Arc::new(sites);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        let s_clone = Arc::clone(&sites);

        pool.execute(move || {
            handle_connection(stream, s_clone);
        });
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream, sites: Arc<Vec<&str>>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let url = String::from_utf8_lossy(&buffer);
    let mut keyword = url.split_ascii_whitespace();
    keyword.next();

    let keyword = keyword.next().unwrap();
    let keyword = &keyword[1..];

    let index = sites.binary_search(&keyword);
    let (status_line, filename) = match index {
        Ok(ind) => {
            let s1 = sites[ind].to_owned();
            ("HTTP/1.1 200 OK", s1 + ".html")
        }
        Err(_) => ("HTTP/1.1 404 NOT FOUND", "404.html".to_string()),
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
