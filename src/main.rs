mod config;
mod graph;
mod structs;

use crate::graph::Graph;
use crate::structs::Node;
use crate::config::Config;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let _config = Config::new();
    let node = Node::new("First node".to_string());

    let mut graph = Graph::new("mygraph".to_string());
    graph.insert(node);
    graph.list();
    println!("{}", graph);
    serve();
}

fn serve() {
    println!("starting server");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(stream);
    }
}

fn handle(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
