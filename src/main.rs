pub mod pagerank;

use pagerank::Graph;

fn main() {
    let mut g = Graph::new();
    g.add_link(0,1,0.4);
    g.add_link(0,1,0.3);
    println!("{}",g.nodes[&0]);
}
