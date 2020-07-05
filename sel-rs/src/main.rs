use petgraph::prelude::Graph;
use petgraph::visit::DfsPostOrder;
use petgraph::dot::{Dot, Config};

#[derive(Clone, Debug)]
pub enum Opcode {
    Ref,
    Add,
    Mul,
    Reg,
    Any,
}

impl PartialEq for Opcode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Opcode::Any, _) => true,
            (_, Opcode::Any) => true,
            (Opcode::Ref, Opcode::Ref) => true,
            (Opcode::Add, Opcode::Add) => true,
            (Opcode::Mul, Opcode::Mul) => true,
            (Opcode::Reg, Opcode::Reg) => true,
            (_, _) => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Loc {
    Gen,
    Dsp,
    Lut,
}

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    opcode: Opcode,
    loc: Loc,
}

impl Node {
    pub fn new(name: &str, opcode: Opcode, loc: Loc) -> Node {
        Node {
            name: name.to_string(),
            opcode: opcode,
            loc: loc,
        }
    }

    pub fn new_gen_ref(name: &str) -> Node {
        Node {
            name: name.to_string(),
            opcode: Opcode::Ref,
            loc: Loc::Gen,
        }
    }

    pub fn new_gen_add(name: &str) -> Node {
        Node {
            name: name.to_string(),
            opcode: Opcode::Add,
            loc: Loc::Gen,
        }
    }

    pub fn new_gen_mul(name: &str) -> Node {
        Node {
            name: name.to_string(),
            opcode: Opcode::Mul,
            loc: Loc::Gen,
        }
    }

    pub fn new_dsp_mul(name: &str) -> Node {
        Node {
            name: name.to_string(),
            opcode: Opcode::Mul,
            loc: Loc::Dsp,
        }
    }

    pub fn opcode(&self) -> &Opcode {
        &self.opcode
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let mut graph = Graph::<Node, ()>::new();
    let a = graph.add_node(Node::new_gen_ref("a"));
    let b = graph.add_node(Node::new_gen_ref("b"));
    let c = graph.add_node(Node::new_gen_ref("c"));
    let t0 = graph.add_node(Node::new_gen_mul("t0"));
    let t1 = graph.add_node(Node::new_gen_add("t1"));

    graph.add_edge(t0, a, ());
    graph.add_edge(t0, b, ());
    graph.add_edge(t1, t0, ());
    graph.add_edge(t1, c, ());

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let mut dfs = DfsPostOrder::new(&graph, t1);

    while let Some(visited) = dfs.next(&graph) {
        if let Some(node) = graph.node_weight_mut(visited) {
            if *node.opcode() == Opcode::Mul {
                *node = Node::new_dsp_mul(node.name());
            }
        }
    }

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}
