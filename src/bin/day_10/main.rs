use std::{collections::VecDeque, isize, time};

use petgraph::{graph::NodeIndex, visit::Bfs, Graph};

struct Node {
    height: isize,
}

struct DiGraph {
    graph: Graph<Node, ()>,
    zeroes: Vec<NodeIndex>,
}

impl DiGraph {
    fn parse(contents: &str) -> Self {
        let mut graph = Graph::new();

        let mut up = VecDeque::new();
        let mut zeroes = vec![];

        for x in contents.trim().lines() {
            let mut left = None;

            for y in x.trim().chars() {
                let current = graph.add_node(Node::new(y));

                // Keep track of entry points
                if graph[current].height == 0 {
                    zeroes.push(current);
                }

                match left {
                    Some(node) => {
                        Self::add_node_if_adjacent(&mut graph, current, node);
                    }
                    None => {}
                };

                // when buffer is populated the front node is above current
                if up.len() == x.len() {
                    let up = up.pop_front();
                    assert!(up.is_some());

                    Self::add_node_if_adjacent(&mut graph, current, up.unwrap());
                }

                // keep track of neighbors
                up.push_back(current);
                left = Some(current);
            }
        }

        Self { graph, zeroes }
    }

    fn add_node_if_adjacent(graph: &mut Graph<Node, ()>, current: NodeIndex, other: NodeIndex) {
        let (min, max) = if graph[current].height > graph[other].height {
            (other, current)
        } else {
            (current, other)
        };

        if (graph[max].height - graph[min].height) == 1 {
            _ = graph.add_edge(min, max, ());
        }
    }

    fn count_paths(&self) -> usize {
        let mut count = 0;
        for zero in &self.zeroes {
            let mut bfs = Bfs::new(&self.graph, *zero);

            while let Some(nx) = bfs.next(&self.graph) {
                if self.graph[nx].height == 9 {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_path_scores(&self) -> usize {
        let mut count = 0;
        for zero in &self.zeroes {
            let mut stack = vec![(zero.clone(), vec![zero.clone()])];

            while let Some((current, path)) = stack.pop() {
                if self.graph[current].height == 9 {
                    count += 1
                } else {
                    for neighbor in self.graph.neighbors(current) {
                        if path.contains(&neighbor) {
                            continue;
                        }

                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        stack.push((neighbor, new_path));
                    }
                }
            }
        }
        count
    }
}

impl Node {
    fn new(height: char) -> Self {
        let height = if !height.is_numeric() {
            // -2 so that it's not adjacent to 0
            -2
        } else {
            height as isize - '0' as isize
        };

        Self { height }
    }
}

fn main() -> Result<(), std::io::Error> {
    let contents = std::fs::read_to_string("src/inputs/day10.txt")?;

    let di_graph = DiGraph::parse(&contents);

    // Part 1
    let start = time::Instant::now();
    println!(
        "Part 1: count = {}, completed in = {:?}\n",
        di_graph.count_paths(),
        start.elapsed()
    );

    // Part 2
    let start = time::Instant::now();
    println!(
        "Part 2: count = {}, completed in = {:?}\n",
        di_graph.count_path_scores(),
        start.elapsed()
    );

    Ok(())
}
