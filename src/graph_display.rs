use std::fmt;

use crate::gnode::GNode;
use crate::gnode::GNodeId;

use super::Graph;

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Graph {{\n")?;

        // Indent for nodes list
        write!(f, "    nodes: [\n")?;
        let mut node_strings = Vec::new();
        for node in self.nodes.values() {
            node_strings.push(format!("        {}", node));
        }
        write!(f, "{}\n", node_strings.join(",\n"))?;
        write!(f, "    ],\n")?;

        // Indent for edges list
        write!(f, "    edges: [\n")?;
        let mut edge_strings = Vec::new();
        for (from, to_list) in self.edges.iter() {
            let from_string = format!("{}", from);
            let to_strings: Vec<String> = to_list.iter().map(|id| format!("{}", id)).collect();
            let edge_string = format!("        {} -> [{}]", from_string, to_strings.join(", "));
            edge_strings.push(edge_string);
        }
        write!(f, "{}\n", edge_strings.join(",\n"))?;
        write!(f, "    ],\n")?;

        // Indent for reverse_edges list
        write!(f, "    reverse_edges: [\n")?;
        let mut reverse_edge_strings = Vec::new();
        for (to, from_list) in self.reverse_edges.iter() {
            let to_string = format!("{}", to);
            let from_strings: Vec<String> = from_list.iter().map(|id| format!("{}", id)).collect();
            let reverse_edge_string = format!("        {} <- [{}]", to_string, from_strings.join(", "));
            reverse_edge_strings.push(reverse_edge_string);
        }
        write!(f, "{}\n", reverse_edge_strings.join(",\n"))?;
        write!(f, "    ]\n")?;

        write!(f, "}}")
    }
}
