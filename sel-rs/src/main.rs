use std::rc::Rc;
use std::fmt;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Opcode {
    Ref,
    Add,
    Mul,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Ref => write!(f, "ref"),
            Opcode::Add => write!(f, "add"),
            Opcode::Mul => write!(f, "mul"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    opcode: Opcode,
    lhs: Option<Rc<Node>>,
    rhs: Option<Rc<Node>>,
    value: Option<Instr>,
}

impl Node {
    pub fn new_with_name_and_opcode(name: &str, opcode: Opcode) -> Node {
        Node {
            name: name.to_string(),
            opcode: opcode,
            lhs: None,
            rhs: None,
            value: None,
        }
    }

    pub fn change_opcode(&mut self, opcode: Opcode) {
        self.opcode = opcode;
    }

    pub fn change_lhs(&mut self, node: &Node) {
        self.lhs = Some(Rc::new(node.clone()));
    }

    pub fn change_rhs(&mut self, node: &Node) {
        self.rhs = Some(Rc::new(node.clone()));
    }

    pub fn is_part_equal(&self, node: &Node) -> bool {
        if self.opcode != node.opcode { // we check for opcode only atm
            false
        } else {
            let leq = match (&self.lhs, node.lhs.as_ref()) {
                (None, None) => true,
                (Some(a), Some(b)) => a.is_part_equal(&b),
                _ => false,
            };
            let req = match (&self.rhs, node.rhs.as_ref()) {
                (None, None) => true,
                (Some(a), Some(b)) => a.is_part_equal(&b),
                _ => false,
            };
            leq && req
        }
    }

    // from https://rosettacode.org/wiki/Tree_traversal#Rust
    pub fn iterative_postorder(&self) -> Vec<&Node> {
        let mut stack: Vec<&Node> = Vec::new();
        let mut res: Vec<&Node> = Vec::new();
        stack.push(self);
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            res.push(node);
            match node.lhs {
                None => (),
                Some(ref n) => stack.push(n),
            }
            match node.rhs {
                None => (),
                Some(ref n) => stack.push(n),
            }
        }
        res.reverse();
        res
    }
}

#[derive(Clone, Debug)]
pub enum Loc {
    Gen,
    Lut,
    Dsp,
}

impl Loc {
    pub fn cost(&self) -> u128 {
        match self {
            Loc::Gen => 3,
            Loc::Lut => 2,
            Loc::Dsp => 1,
        }
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Loc::Gen => write!(f, "??"),
            Loc::Dsp => write!(f, "dsp"),
            Loc::Lut => write!(f, "lut"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Instr {
    opcode: Opcode,
    dst: String,
    lhs: String,
    rhs: String,
    loc: Loc,
}

impl Instr {
    pub fn new_gen_instr(opcode: Opcode, dst: &str, lhs: &str, rhs: &str) -> Instr {
        Instr {
            opcode: opcode,
            dst: dst.to_string(),
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            loc: Loc::Gen,
        }
    }

    pub fn new_dsp_instr(opcode: Opcode, dst: &str, lhs: &str, rhs: &str) -> Instr {
        Instr {
            opcode: opcode,
            dst: dst.to_string(),
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            loc: Loc::Dsp,
        }
    }

    pub fn new_lut_instr(opcode: Opcode, dst: &str, lhs: &str, rhs: &str) -> Instr {
        Instr {
            opcode: opcode,
            dst: dst.to_string(),
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            loc: Loc::Lut,
        }
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {} @{}", self.opcode, self.dst, self.lhs, self.rhs, self.loc)
    }
}

#[derive(Clone, Debug)]
pub struct Prog {
    body: Vec<Instr>,
}

impl Prog {
    pub fn new() -> Prog {
        Prog {
            body: Vec::new(),
        }
    }

    pub fn create_gen_instr(&mut self, opcode: Opcode, dst: &str, lhs: &str, rhs: &str) {
        self.body.push(Instr::new_gen_instr(opcode, dst, lhs, rhs));
    }

    pub fn create_dsp_instr(&mut self, opcode: Opcode, dst: &str, lhs: &str, rhs: &str) {
        self.body.push(Instr::new_dsp_instr(opcode, dst, lhs, rhs));
    }

    pub fn create_lut_instr(&mut self, opcode: Opcode, dst: &str, lhs: &str, rhs: &str) {
        self.body.push(Instr::new_lut_instr(opcode, dst, lhs, rhs));
    }
}

impl fmt::Display for Prog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut body = String::new();
        for instr in self.body.iter() {
            body.push_str(&format!("\n{};", instr.to_string()));
        }
        write!(f, "{}", body)
    }
}

fn create_program() -> Prog {
    let mut prog = Prog::new();
    prog.create_gen_instr(Opcode::Mul, "t0", "a", "b");
    prog.create_gen_instr(Opcode::Add, "t1", "t0", "c");
    prog
}

type DAG = HashMap<String, Node>;

fn create_dag_from_prog(prog: &Prog) -> DAG {
    let mut dag = DAG::new();
    for instr in prog.body.iter() {
        if !dag.contains_key(&instr.lhs) {
            let lhs = Node::new_with_name_and_opcode(&instr.lhs, Opcode::Ref);
            dag.insert(instr.lhs.to_string(), lhs.clone());
        }
        if !dag.contains_key(&instr.rhs) {
            let rhs = Node::new_with_name_and_opcode(&instr.rhs, Opcode::Ref);
            dag.insert(instr.rhs.to_string(), rhs.clone());
        }
        if !dag.contains_key(&instr.dst) {
            let mut op = Node::new_with_name_and_opcode(&instr.dst, instr.opcode.clone());
            op.change_lhs(&dag.remove(&instr.lhs).expect(&format!("Error: {} is not found, perhaps was used already", &instr.lhs)));
            op.change_rhs(&dag.remove(&instr.rhs).expect(&format!("Error: {} is not found, perhaps was used already", &instr.rhs)));
            dag.insert(instr.dst.to_string(), op.clone());
        }
    }
    dag
}

fn main() {
    let prog = create_program();
    println!("{}", prog);
    let dag = create_dag_from_prog(&prog);
    let nodes = dag.get("t1").unwrap().iterative_postorder();
    for n in nodes.iter() {
        println!("n:{:?}", n);
    }
}
