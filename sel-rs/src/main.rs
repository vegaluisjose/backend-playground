use std::rc::Rc;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    lhs: Option<Rc<Node>>,
    rhs: Option<Rc<Node>>,
    value: Option<Instr>,
    cost: u128,
}

impl Node {
    pub fn new_with_name(name: &str) -> Node {
        Node {
            name: name.to_string(),
            lhs: None,
            rhs: None,
            value: None,
            cost: 0,
        }
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
    Dsp,
    Lut,
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

fn main() {
    let prog = create_program();
    println!("{}", prog);
}
