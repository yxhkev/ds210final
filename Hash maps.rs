use std::collections::HashMap;

struct Node {
    data: i32,
    edges: Vec<i32>,
}

struct Graph {
    nodes: HashMap<i32, Node>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, node_id: i32, data: i32) {
        self.nodes.insert(
            node_id,
            Node {
                data,
                edges: Vec::new(),
            },
        );
    }

    fn add_edge(&mut self, from_node_id: i32, to_node_id: i32) {
        if let Some(node) = self.nodes.get_mut(&from_node_id) {
            node.edges.push(to_node_id);
        }
    }
}

fn main() {
    let mut graph = Graph::new();

    graph.add_node(1, 10);
    graph.add_node(2, 20);
    graph.add_node(3, 30);

    graph.add_edge(1, 2);
    graph.add_edge(2, 3);
}