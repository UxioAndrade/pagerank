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

    pub fn page_rank(&mut self, alpha: f64, epsilon: f64,mut callback: impl FnMut(u32, f64)) {
        
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
                
#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.00001;

    #[test]
    fn test_empty() {
        let mut g = Graph::new();
        let mut actual : HashMap<u32, f64> = HashMap::new();
        let expected : HashMap<u32, f64> = HashMap::new();

        g.page_rank(0.85, 0.000001, |node, rank| {actual.insert(node,rank);});
        assert_eq!(actual,expected);
    }
    
    #[test]
    fn test_simple() {
        let mut g = Graph::new();
        g.add_link(1,2,1.0);
        g.add_link(1,3,1.0);
        g.add_link(2,3,1.0);
        g.add_link(2,4,1.0);
        g.add_link(3,1,1.0);
        
        let mut actual : HashMap<u32,f64> = HashMap::new();
        g.page_rank(0.85, 0.000001, |node, rank| {actual.insert(node,rank);});

        assert!((actual[&1]-0.32721836185043207).abs() < EPSILON);
        assert!((actual[&2]-0.2108699481253495).abs() < EPSILON);
        assert!((actual[&3]-0.3004897566512289).abs() < EPSILON);
        assert!((actual[&4]-0.16142193337298952).abs() < EPSILON);
    }

    #[test]
    fn test_weighted() { 
        
        let mut g = Graph::new();
        g.add_link(1,2,1.0);
        g.add_link(1,3,2.0);
        g.add_link(2,3,3.0);
        g.add_link(2,4,4.0);
        g.add_link(3,1,5.0);
        
        let mut actual : HashMap<u32,f64> = HashMap::new();
        g.page_rank(0.85, 0.000001, |node, rank| {actual.insert(node,rank);});
 
        assert!((actual[&1]-0.34983779905464363).abs() < EPSILON);
        assert!((actual[&2]-0.1688733284604475).abs() < EPSILON);
        assert!((actual[&3]-0.3295121849483849).abs() < EPSILON);
        assert!((actual[&4]-0.15177668753652385).abs() < EPSILON);
    }


    #[test]
    fn test_duplicates() { 
        
        let mut g = Graph::new();
        g.add_link(1,2,1.0);
        g.add_link(1,3,2.0);
        g.add_link(2,3,3.0);
        g.add_link(2,4,4.0);
        g.add_link(3,1,5.0);
        
        g.add_link(1,2,6.0);
        g.add_link(1,3,7.0);

        let mut actual : HashMap<u32,f64> = HashMap::new();
        g.page_rank(0.85, 0.000001, |node, rank| {actual.insert(node,rank);});

        assert!((actual[&1]-0.3312334209098247).abs() < EPSILON);
        assert!((actual[&2]-0.19655848316544225).abs() < EPSILON);
        assert!((actual[&3]-0.3033555769882879).abs() < EPSILON);
        assert!((actual[&4]-0.168852518936445).abs() < EPSILON);
        
    }

}
