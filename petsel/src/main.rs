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
    cost: u128,
    opcode: Opcode,
    operands: Vec<Node>,
    width: u64,
    target: Option<String>,
}

impl Node {
    pub fn new_with_attrs(opcode: &Opcode, width: u64, cost: u128) -> Node {
        Node {
            opcode: opcode.clone(),
            operands: Vec::new(),
            width: width.clone(),
            cost: cost,
            target: None,
        }
    }

    pub fn change_cost(&mut self, cost: u128) -> &mut Node {
        self.cost = cost;
        self
    }

    pub fn push_operand(&mut self, operand: &Node) -> &mut Node {
        self.operands.push(operand.clone());
        self
    }

    pub fn was_visited(&self) -> bool {
        match self.target {
            None => false,
            Some(_) => true,
        }
    }

    pub fn postorder(&self) -> Vec<Node> {
        let mut stack : Vec<Node> = Vec::new();
        let mut res : Vec<Node> = Vec::new();
        stack.push(self.clone());
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            res.push(node.clone());
            for operand in node.operands.iter() {
                stack.push(operand.clone());
            }
        }
        res.reverse();
        res
    }
}

fn main() {
    let input_a = Node::new_with_attrs(&Opcode::Input, 8, 0);
    let input_b = Node::new_with_attrs(&Opcode::Input, 8, 1);
    let mut add = Node::new_with_attrs(&Opcode::Add, 8, 1);
    add.push_operand(&input_a);
    add.push_operand(&input_b);
    let x = add.postorder();
    println!("{:?}", x);
}
