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

#[derive(Clone, Debug)]
pub struct Pattern {
    pat: Vec<Opcode>,
    cost: u128,
    loc: Loc,
}

impl Pattern {
    pub fn new(cost: u128, loc: Loc) -> Pattern {
        Pattern {
            pat: Vec::new(),
            cost: cost,
            loc: loc,
        }
    }

    pub fn push_op(&mut self, op: Opcode) {
        self.pat.push(op);
    }
}

fn pat_0() -> Pattern {
    let mut pat = Pattern::new(10, Loc::Dsp);
    pat.push_op(Opcode::Any);
    pat.push_op(Opcode::Any);
    pat.push_op(Opcode::Mul);
    pat.push_op(Opcode::Any);
    pat.push_op(Opcode::Add);
    pat
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

    let mut root = DfsPostOrder::new(&graph, t1);
    while let Some(idx) = root.next(&graph) {
        let p0 = pat_0();
        let mut pat_ops = p0.pat.iter();
        // check if there is a pattern match
        let mut pat_match: bool = true;
        let mut subgraph = DfsPostOrder::new(&graph, idx);
        while let Some(pat_op) = pat_ops.next() {
            if let Some(sub_idx) = subgraph.next(&graph) {
                if let Some(node) = graph.node_weight(sub_idx) {
                    if node.opcode != *pat_op {
                        pat_match = false;
                    }
                }
            } else {
                pat_match = false;
                break;
            }
        }
        if pat_match && pat_ops.len() == 0 { // check all nodes in the pattern
            if let Some(node) = graph.node_weight(idx) {
                println!("This node is a candidate: {:?}", node);
            }
        }
    }
}
