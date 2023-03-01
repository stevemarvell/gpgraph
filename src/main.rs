mod gnode;
mod gnode_builder;
mod graph;
mod graph_display;

use gnode_builder::GNodeBuilder;
use gnode::GNode;
use graph::Graph;
use graph::ToAscii;

fn main() {
    // Create some nodes using the GNodeBuilder
    let mut builder = GNodeBuilder::new();
    let node1 = builder.number1(123).number2(456).build();
    let node2 = builder.number1(789).number2(101112).build();
    let node3 = builder.number1(131415).number2(161718).build();
    let node4 = builder.number1(192021).number2(222324).build();

    // Create a new Graph and add the nodes
    let mut graph = Graph::new();
    graph.add_node(node1);
    graph.add_node(node2);
    graph.add_node(node3);

    // Add some edges to the Graph
    graph.add_edge(node1.id, node2.id);
    graph.add_edge(node1.id, node3.id);
    graph.add_edge(node3.id, node2.id);

    // Remove a node from the Graph
    graph.remove_node(&node2);

    // Add another node and edge to the Graph
    graph.add_node(node4);
    graph.add_edge(node1.id, node4.id);

    println!("{}", graph);
    println!("{}", graph.to_ascii());
}
