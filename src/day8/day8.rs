use std::collections::HashMap;
use std::str::FromStr;
use std::{error::Error, fs};

fn main() {
    let day8_res = solve_day8();
    println!(
        "Day 5 Result:\n\tPart 1: {}\n\tPart 2: {}",
        day8_res.0.unwrap(),
        day8_res.1.unwrap()
    );
}

#[derive(Eq, PartialEq)]
struct Node<T> {
    label: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(label: T) -> Self {
        Node {
            label,
            left: None,
            right: None,
        }
    }
}

struct NodeParseError;

impl FromStr for Node<String> {
    type Err = NodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let label: String = s
            .trim()
            .split_once(' ')
            .ok_or(NodeParseError)?
            .0
            .to_string();
        let (left, right) = s
            .split_once('(')
            .ok_or(NodeParseError)?
            .1
            .split_once(')')
            .ok_or(NodeParseError)?
            .0
            .split_once(", ")
            .ok_or(NodeParseError)?;
        Ok(Node {
            label,
            left: None,
            right: None,
        })
    }
}

/// Graph structure where the key is the label and the value is the node
pub struct Graph {
    nodes: HashMap<String, Node<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    /// Insert a node into the graph with the label as the key
    pub fn add_node<'a>(&mut self, key: &str) -> &'a mut Node<T> {
        self.nodes
            .entry(key.to_string())
            .or_insert_with(|| Node::new(key.to_string()))
    }

    /// Add a node's children to the graph given the key of the node
    pub fn add_children(&mut self, key: &str, left: &str, right: &str) {
        let left_node = self
            .nodes
            .entry(left.to_string())
            .or_insert_with(|| Node::new(left.to_string()));
        self.nodes
            .entry(key.to_string())
            .and_modify(|n| n.left = Some(Box::new(left_node.clone())));
        let right_node = self
            .nodes
            .entry(left.to_string())
            .or_insert_with(|| Node::new(right.to_string()));
        self.nodes
            .entry(key.to_string())
            .and_modify(|n| n.right = Some(Box::new(right_node.clone())));
    }

    /// Insert a node and children into the graph
    pub fn add_node_and_children(&mut self, key: &str, left: &str, right: &str) {
        self.add_node(key);
        self.add_children(key, left, right);
    }
}

struct GraphParseError;

impl FromStr for Graph {
    type Err = GraphParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!();
    }
}

pub fn solve_day8() -> (Result<u32, Box<dyn Error>>, Result<u32, Box<dyn Error>>) {
    (
        solve_day8_part1("src/day8/input.txt"),
        solve_day8_part2("src/day8/input.txt"),
    )
}

fn solve_day8_part1(input_path: &str) -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string(input_path)?;

    let (commands_str, nodes_str) = contents
        .trim()
        .split_once("\r\n\r\n")
        .expect("input should separate commands from nodes");

    let mut nodes_graph: Graph = Graph::new();

    for line in nodes_str.lines() {
        let label: &str = line.trim().split_once(' ').unwrap().0;
        let (left, right) = line
            .split_once('(')
            .unwrap()
            .1
            .split_once(')')
            .unwrap()
            .0
            .split_once(", ")
            .unwrap();
        nodes_graph.add_node_and_children(&label, &left, &right);
    }

    //
    // for char in commands_str.chars() {
    //     for node in nodes {
    //         // match char {
    //         //     'L' => {}
    //         // }
    //     }
    // }

    todo!();
}

fn solve_day8_part2(input_path: &str) -> Result<u32, Box<dyn Error>> {
    let contents: String = fs::read_to_string(input_path)?;
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_day8_part1_sample() {
        assert_eq!(solve_day8_part1("src/day8/sample-input.txt").unwrap(), 0);
    }

    #[test]
    fn test_solve_day8_part1() {
        assert_eq!(solve_day8_part1("src/day8/input.txt").unwrap(), 0);
    }

    #[test]
    fn test_solve_day8_part2_sample() {
        assert_eq!(solve_day8_part2("src/day8/sample-input.txt").unwrap(), 0);
    }

    #[test]
    fn test_solve_day8_part2() {
        assert_eq!(solve_day8_part2("src/day8/input.txt").unwrap(), 0);
    }
}
