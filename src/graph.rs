use std::fmt;
use crate::structs::Node;

pub struct Graph {
    name: String,
    nodes: Vec<Node>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "graph '{}'", self.name)
    }
}

impl Graph {
    pub fn new(name: String) -> Graph {
        Graph {
            name: name,
            nodes: Vec::new(),
        }
    }
    pub fn insert(&mut self, n: Node) {
        self.nodes.push(n);

    }
    pub fn list(&self) {
        for n in self.nodes.iter() {
            println!("\t[Node]: {}", *n);
        }
    }
}