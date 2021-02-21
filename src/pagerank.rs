use std::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub weight: f64,
    pub outbound: f64,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"({}, {})", self.weight, self.outbound)
    }
}

pub struct Graph {
    pub nodes: HashMap<u32,Node>,
    pub edges: HashMap<u32,HashMap<u32,f64>>
}

impl Node {
    pub fn new() -> Node {
        Node {
            weight: 0.0,
            outbound: 0.0,
        }
    }
}


impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    fn insert_new_node(&mut self,index: u32) {
        self.nodes.insert(
            index,
            Node::new(),
        );
    }

    pub fn add_link(&mut self, source: u32, target: u32, weight: f64) {
        if !self.nodes.contains_key(&source) {
            self.insert_new_node(source)
        }
        self.nodes.get_mut(&source).unwrap().outbound += weight;
        if !self.nodes.contains_key(&target) {
            self.insert_new_node(target)
        }
        if !self.edges.contains_key(&source) {
            self.edges.insert(
                source,
                HashMap::new(),
            );
        };
        if !self.edges[&source].contains_key(&target) {
            self.edges.get_mut(&source).unwrap().insert(
                target,
                weight,
            );
        } else {
            *self.edges.get_mut(&source).unwrap().get_mut(&target).unwrap() += weight;
        }
    }
}
                

