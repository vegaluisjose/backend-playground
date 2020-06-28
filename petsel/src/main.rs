use petgraph::Graph;
use petgraph::dot::{Dot, Config};
use std::fs::File;
use std::io::Write;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Opcode {
    Input,
    Add,
    Mul,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Input => write!(f, "input"),
            Opcode::Add => write!(f, "add"),
            Opcode::Mul => write!(f, "mul"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    opcode: Opcode,
    width: u64,
}

impl Node {
    pub fn new_input(width: u64) -> Node {
        Node {
            opcode: Opcode::Input,
            width: width,
        }
    }

    pub fn new_op(op: Opcode, width: u64) -> Node {
        Node {
            opcode: op,
            width: width,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.opcode)
    }
}

#[allow(dead_code)]
fn write_dag(source: &Graph<Node, &str, petgraph::Directed>, name: &str) {
    let mut f = File::create(format!("{}.dot", name)).expect("Error: creating the file");
    let output = format!("{}", Dot::with_config(source, &[Config::EdgeNoLabel]));
    f.write_all(&output.as_bytes()).expect("Error: writing to the file");
}

fn main() {
    let mut dag : Graph<Node, _, petgraph::Directed> = Graph::new();
    let input_a = dag.add_node(Node::new_input(8));
    let input_b = dag.add_node(Node::new_input(8));
    let add = dag.add_node(Node::new_op(Opcode::Add, 8));
    dag.add_edge(input_a, add, "");
    dag.add_edge(input_b, add, "");
}
