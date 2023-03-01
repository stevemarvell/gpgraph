use std::collections::HashMap;
use std::fmt;

use crate::gnode::{GNode, GNodeId};

pub struct Graph {
    pub nodes: HashMap<GNodeId, GNode>,
    pub edges: HashMap<GNodeId, Vec<GNodeId>>,
    pub(crate) reverse_edges: HashMap<GNodeId, Vec<GNodeId>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: GNode) {
        let node_id = node.id;
        self.nodes.insert(node_id, node);
    }

    pub fn add_edge(&mut self, from: GNodeId, to: GNodeId) {
        self.edges.entry(from).or_insert_with(Vec::new).push(to);
        self.reverse_edges
            .entry(to)
            .or_insert_with(Vec::new)
            .push(from);
    }

    pub fn remove_node(&mut self, node: &GNode) {
        let node_id = node.id;

        // Remove the node from the nodes HashMap
        self.nodes.remove(&node_id);

        // Remove any edges that start at the node
        if let Some(edges) = self.edges.remove(&node_id) {
            for to in &edges {
                if let Some(reverse_edges) = self.reverse_edges.get_mut(to) {
                    reverse_edges.retain(|&id| id != node_id);
                    if edges.is_empty() {
                        self.edges.remove(&to);
                    }
                }
            }
        }

        // Remove any edges that end at the node
        if let Some(reverse_edges) = self.reverse_edges.remove(&node_id) {
            for from in &reverse_edges {
                if let Some(edges) = self.edges.get_mut(from) {
                    edges.retain(|&id| id != node_id);
                    if edges.is_empty() {
                        self.edges.remove(&from);
                    }
                }
            }
        }
    }

    pub fn remove_edge(&mut self, from: GNodeId, to: GNodeId) {
        if let Some(edges) = self.edges.get_mut(&from) {
            edges.retain(|&id| id != to);
            if edges.is_empty() {
                self.edges.remove(&from);
            }
        }
        if let Some(edges) = self.reverse_edges.get_mut(&to) {
            edges.retain(|&id| id != from);
            if edges.is_empty() {
                self.reverse_edges.remove(&to);
            }
        }
    }

    pub fn get_nodes(&self) -> Vec<&GNode> {
        self.nodes.values().collect()
    }

    pub fn get_neighbors(&self, node: &GNode) -> Vec<&GNode> {
        let mut neighbors = Vec::new();
        if let Some(edges) = self.edges.get(&node.id) {
            for neighbor_id in edges {
                if let Some(neighbor) = self.nodes.get(neighbor_id) {
                    neighbors.push(neighbor);
                }
            }
        }
        neighbors
    }
}

pub(crate) trait ToAscii {
    fn to_ascii(&self) -> String;
}

impl ToAscii for Graph {
    fn to_ascii(&self) -> String {
        let mut lines = vec![];

        for (from, to_list) in self.edges.iter() {
            let from_string = format!("{}", from);
            for to in to_list {
                let to_string = format!("{}", to);
                let line = format!("{} -> {}", from_string, to_string);
                lines.push(line);
            }
        }

        lines.join("\n")
    }
}

