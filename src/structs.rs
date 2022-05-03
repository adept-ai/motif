use std::fmt;
use uuid::Uuid;

pub struct Node {
    pub name: String,
    pub id: String
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "node '{}' {}", self.name, self.id)
    }
}

impl Node {
    pub fn new(name: String) -> Node {
        Node {
            name: name,
            id: Uuid::new_v4().to_string(),
        }
    }
}
