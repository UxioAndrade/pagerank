# pagerank

Rust implementation of the Weighted PageRank algorithm 

## Usage

```rust
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
```

## Output

```
Node: 1 - Rank: 0.34983779905464363
Node: 2 - Rank: 0.1688733284604475
Node: 3 - Rank: 0.3295121849483849
Node: 4 - Rank: 0.15177668753652385
```

This repo is a Rust implementation of the algorithm described in the following [repository](https://github.com/alixaxel/pageranki)  (which is in Go) 
