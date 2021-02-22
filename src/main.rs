pub mod pagerank;

use pagerank::Graph;

fn main() {
    let mut g = Graph::new();
    g.add_link(1,2,1.0); 
    g.add_link(1,3,2.0);
    g.add_link(2,3,3.0);
    g.add_link(2,4,4.0);
    g.add_link(3,1,5.0);
    g.page_rank(0.85, 0.000001, |node, rank| {println!("Node: {} - Rank: {}",node,rank); });
}
