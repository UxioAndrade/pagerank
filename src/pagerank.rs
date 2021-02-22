use std::fmt;
use std::collections::HashMap;

const DEFAULT_DELTA: f64 = 1.0;

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

    pub fn page_rank(&mut self, alpha: f64, epsilon: f64, callback: fn(u32, f64)) {
        
        let mut delta = DEFAULT_DELTA;
        let inverse = 1.0/(self.nodes.len()) as f64;
        let nodes = &self.nodes;
        
        self.edges.iter_mut()
            .filter(|(k,_)| nodes[k].outbound > 0.0)
            .for_each(|(k,v)| v.iter_mut().for_each(|(_,y)| *y /= nodes[k].outbound));
        
        self.nodes.iter_mut()
            .for_each(|(_,v)| v.weight = inverse);

        while delta > epsilon{

            let mut leak = 0.0 as f64;
            let mut tmp_nodes : HashMap<u32,f64> = HashMap::new();
            self.nodes.iter_mut()
                .for_each(|(k,v)| {
                    tmp_nodes.insert(*k,v.weight);
                    if v.outbound == 0.0 {
                        leak += v.weight;
                    }
                    v.weight = 0.0;
		        });
            
            leak *= alpha;

            let keys = self.nodes.keys().cloned().collect::<Vec<u32>>(); 
            for k in keys {
                if self.edges.contains_key(&k){
                    for (x,y) in self.edges[&k].iter() {
                        self.nodes.get_mut(&x).unwrap().weight += alpha*tmp_nodes[&k]*(*y);
                    }
                }
                self.nodes.get_mut(&k).unwrap().weight += (1.0 - alpha)*inverse + leak*inverse;
            }
            
            delta = 0.0;
            
            self.nodes.iter()
                .for_each(|(k,v)| if tmp_nodes.contains_key(k) {delta += f64::abs(v.weight - tmp_nodes[k])});
		}
         
        self.nodes.iter()
            .for_each(|(k,v)| callback(*k,v.weight));
    }
}
                

