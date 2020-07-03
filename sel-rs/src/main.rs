use std::rc::Rc;
use std::fmt;
use std::collections::HashMap;

pub type DAG = HashMap<String, Node>;

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    opcode: Option<String>,
    lhs: Option<Rc<Node>>,
    rhs: Option<Rc<Node>>,
    value: Option<Instr>,
    cost: u128,
}

impl Node {
    pub fn new_with_name(name: &str) -> Node {
        Node {
            name: name.to_string(),
            opcode: None,
            lhs: None,
            rhs: None,
            value: None,
            cost: 0,
        }
    }

    pub fn change_opcode(&mut self, opcode: &str) {
        self.opcode = Some(opcode.to_string());
    }

    pub fn change_cost(&mut self, cost: u128) {
        self.cost = cost;
    }

    pub fn is_isomorphic(&self, node: &Node) -> bool {
        if self.name != node.name {
            false
        } else {
            let leq = match (&self.lhs, node.lhs.as_ref()) {
                (None, None) => true,
                (Some(a), Some(b)) => a.is_isomorphic(&b),
                _ => false,
            };
            let req = match (&self.rhs, node.rhs.as_ref()) {
                (None, None) => true,
                (Some(a), Some(b)) => a.is_isomorphic(&b),
                _ => false,
            };
            leq && req
        }
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
    opcode: String,
    dst: String,
    lhs: String,
    rhs: String,
    loc: Loc,
}

impl Instr {
    pub fn new_gen_instr(opcode: &str, dst: &str, lhs: &str, rhs: &str) -> Instr {
        Instr {
            opcode: opcode.to_string(),
            dst: dst.to_string(),
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            loc: Loc::Gen,
        }
    }

    pub fn new_dsp_instr(opcode: &str, dst: &str, lhs: &str, rhs: &str) -> Instr {
        Instr {
            opcode: opcode.to_string(),
            dst: dst.to_string(),
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            loc: Loc::Dsp,
        }
    }

    pub fn new_lut_instr(opcode: &str, dst: &str, lhs: &str, rhs: &str) -> Instr {
        Instr {
            opcode: opcode.to_string(),
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

    pub fn create_gen_instr(&mut self, opcode: &str, dst: &str, lhs: &str, rhs: &str) {
        self.body.push(Instr::new_gen_instr(opcode, dst, lhs, rhs));
    }

    pub fn create_dsp_instr(&mut self, opcode: &str, dst: &str, lhs: &str, rhs: &str) {
        self.body.push(Instr::new_dsp_instr(opcode, dst, lhs, rhs));
    }

    pub fn create_lut_instr(&mut self, opcode: &str, dst: &str, lhs: &str, rhs: &str) {
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
    prog.create_gen_instr("mul", "t0", "a", "b");
    prog.create_gen_instr("add", "t1", "t0", "c");
    prog
}

fn create_dag_from_prog(prog: &Prog) -> DAG {
    let mut dag = DAG::new();
    for instr in prog.body.iter() {
        if !dag.contains_key(&instr.lhs) {
            let mut lhs = Node::new_with_name(&instr.lhs);
            lhs.change_opcode("ref");
            lhs.change_cost(0);
            dag.insert(instr.lhs.to_string(), lhs.clone());
        }
        if !dag.contains_key(&instr.rhs) {
            let mut rhs = Node::new_with_name(&instr.rhs);
            rhs.change_opcode("ref");
            rhs.change_cost(0);
            dag.insert(instr.rhs.to_string(), rhs.clone());
        }
        if !dag.contains_key(&instr.dst) {
            let mut op = Node::new_with_name(&instr.dst);
            op.change_opcode(&instr.opcode);
            op.change_cost(instr.loc.cost());
            dag.insert(instr.dst.to_string(), op.clone());
        }
    }
    dag
}

fn main() {
    let prog = create_program();
    println!("{}", prog);
    let dag = create_dag_from_prog(&prog);
    for (_, value) in dag.iter() {
        println!("{:?}", value);
    }
}
