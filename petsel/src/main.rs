use std::fmt;

#[derive(Clone, Debug, PartialEq)]
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
    visited: bool,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        if self.opcode != other.opcode {
            false
        } else if self.width != other.width {
            false
        } else if self.operands.len() != other.operands.len() {
            false
        } else {
            let mut eq = true;
            for (a, b) in self.operands.iter().zip(other.operands.iter()) {
                if a.opcode != b.opcode {
                    eq = false;
                    break;
                } else if a.width != b.width {
                    eq = false;
                    break;
                } else if a.operands.len() != b.operands.len() {
                    eq = false;
                    break;
                }
            }
            eq
        }
    }
}

impl Eq for Node {}

impl Node {
    pub fn new_with_attrs(opcode: &Opcode, width: u64, loc: &Loc, cost: u128) -> Node {
        Node {
            opcode: opcode.clone(),
            operands: Vec::new(),
            width: width.clone(),
            loc: loc.clone(),
            cost: cost,
            visited: false,
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
        self.visited
    }

    pub fn postorder(&self) -> Vec<Node> {
        let mut stack: Vec<Node> = Vec::new();
        let mut res: Vec<Node> = Vec::new();
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
        write!(
            f,
            "opcode:{} loc:{} cost:{}",
            self.opcode, self.loc, self.cost
        )
    }
}

fn main() {
    let input = Node::new_with_attrs(&Opcode::Input, 8, &Loc::IO, 0);
    let mut lut_add = Node::new_with_attrs(&Opcode::Add, 8, &Loc::Lut, 4);
    let mut dsp_add = Node::new_with_attrs(&Opcode::Add, 8, &Loc::Dsp, 1);
    dsp_add.push_operand(&input);
    dsp_add.push_operand(&input);
    lut_add.push_operand(&input);
    lut_add.push_operand(&input);
    let mut patterns: Vec<Node> = Vec::new();
    patterns.push(input.clone());
    patterns.push(dsp_add.clone());
    println!("{}", dsp_add == lut_add);
}
