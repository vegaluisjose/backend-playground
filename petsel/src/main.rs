use std::rc::Rc;
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
pub enum Loc {
    IO,
    Dsp,
    Lut,
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Loc::IO => write!(f, "io"),
            Loc::Dsp => write!(f, "dsp"),
            Loc::Lut => write!(f, "lut"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    opcode: Opcode,
    operands: Vec<Node>,
    width: u64,
    loc: Loc,
    cost: u128,
    codegen: Option<Rc<Node>>,
}

impl Node {
    pub fn new_with_attrs(opcode: &Opcode, width: u64, loc: &Loc, cost: u128) -> Node {
        Node {
            opcode: opcode.clone(),
            operands: Vec::new(),
            width: width.clone(),
            loc: loc.clone(),
            cost: cost,
            codegen: None,
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
        match self.codegen {
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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "opcode:{} loc:{} cost:{}", self.opcode, self.loc, self.cost)
    }
}

fn main() {
    let input_a = Node::new_with_attrs(&Opcode::Input, 8, &Loc::IO, 0);
    let input_b = Node::new_with_attrs(&Opcode::Input, 8, &Loc::IO, 1);
    let mut add = Node::new_with_attrs(&Opcode::Add, 8, &Loc::Lut, 4);
    add.push_operand(&input_a);
    add.push_operand(&input_b);
    let rev = add.postorder();
    for node in rev.iter() {
        println!("{}", node);
    }
}
