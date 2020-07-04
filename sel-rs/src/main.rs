use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Loc {
    Gen,
    Lut,
    Dsp,
    Equal(String),
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Loc::Gen => write!(f, "??"),
            Loc::Dsp => write!(f, "dsp"),
            Loc::Lut => write!(f, "lut"),
            Loc::Equal(n) => write!(f, "eq({})", n),
        }
    }
}

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
    cur_loc: Loc,
    new_loc: Option<Loc>,
}

impl Node {
    pub fn new_with_attrs(name: &str, opcode: Opcode, loc: Loc) -> Node {
        Node {
            name: name.to_string(),
            opcode: opcode,
            lhs: None,
            rhs: None,
            cur_loc: loc,
            new_loc: None,
        }
    }

    pub fn set_lhs(&mut self, node: &Node) {
        self.lhs = Some(Rc::new(node.clone()));
    }

    pub fn set_rhs(&mut self, node: &Node) {
        self.rhs = Some(Rc::new(node.clone()));
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
        write!(
            f,
            "{} {} {} {} @{}",
            self.opcode, self.dst, self.lhs, self.rhs, self.loc
        )
    }
}

#[derive(Clone, Debug)]
pub struct Prog {
    body: Vec<Instr>,
}

impl Prog {
    pub fn new() -> Prog {
        Prog { body: Vec::new() }
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
            let lhs = Node::new_with_attrs(&instr.lhs, Opcode::Ref, instr.loc.clone());
            dag.insert(instr.lhs.to_string(), lhs.clone());
        }
        if !dag.contains_key(&instr.rhs) {
            let rhs = Node::new_with_attrs(&instr.rhs, Opcode::Ref, instr.loc.clone());
            dag.insert(instr.rhs.to_string(), rhs.clone());
        }
        if !dag.contains_key(&instr.dst) {
            let mut op = Node::new_with_attrs(&instr.dst, instr.opcode.clone(), instr.loc.clone());
            op.set_lhs(&dag.remove(&instr.lhs).expect(&format!(
                "Error: {} is not found, forward reference or was used already",
                &instr.lhs
            )));
            op.set_rhs(&dag.remove(&instr.rhs).expect(&format!(
                "Error: {} is not found, forward reference or was used already",
                &instr.rhs
            )));
            dag.insert(instr.dst.to_string(), op.clone());
        }
    }
    dag
}

#[derive(Clone, Debug)]
pub struct Pattern {
    opcode: Opcode,
    lhs: Option<Rc<Pattern>>,
    rhs: Option<Rc<Pattern>>,
}

impl Pattern {
    pub fn new_with_opcode(opcode: Opcode) -> Pattern {
        Pattern {
            opcode: opcode,
            lhs: None,
            rhs: None,
        }
    }

    pub fn set_lhs(&mut self, pattern: &Pattern) {
        self.lhs = Some(Rc::new(pattern.clone()));
    }

    pub fn set_rhs(&mut self, pattern: &Pattern) {
        self.rhs = Some(Rc::new(pattern.clone()));
    }

    pub fn is_match(&self, node: &Node) -> bool {
        if self.opcode != node.opcode {
            false
        } else {
            let lmatch = match (&self.lhs, node.lhs.as_ref()) {
                (None, _) => true,
                (Some(a), Some(b)) => a.is_match(&b),
                _ => false,
            };
            let rmatch = match (&self.rhs, node.rhs.as_ref()) {
                (None, _) => true,
                (Some(a), Some(b)) => a.is_match(&b),
                _ => false,
            };
            lmatch & rmatch
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tile {
    pattern: Pattern,
    loc: Loc,
    cost: u128,
}

impl Tile {
    pub fn new_with_attrs(pattern: Pattern, loc: Loc, cost: u128) -> Tile {
        Tile {
            pattern: pattern,
            loc: loc,
            cost: cost,
        }
    }
}

fn add_pattern() -> Pattern {
    Pattern::new_with_opcode(Opcode::Add)
}

fn mul_pattern() -> Pattern {
    Pattern::new_with_opcode(Opcode::Mul)
}

fn muladd_pattern() -> Pattern {
    let mut add = add_pattern();
    add.set_lhs(&mul_pattern());
    add
}

fn create_tiles() -> Vec<Tile> {
    vec![
        Tile::new_with_attrs(muladd_pattern(), Loc::Dsp, 1),
        Tile::new_with_attrs(add_pattern(), Loc::Dsp, 2),
        Tile::new_with_attrs(add_pattern(), Loc::Lut, 3),
        Tile::new_with_attrs(mul_pattern(), Loc::Dsp, 1),
    ]
}

fn main() {
    let prog = create_program();
    println!("{}", prog);
    let dag = create_dag_from_prog(&prog);
    let nodes = dag.get("t1").unwrap().iterative_postorder();
    for n in nodes.iter() {
        println!("n:{:?}", n);
    }
    let tiles = create_tiles();
    for t in tiles.iter() {
        println!("{:?}", t);
    }
}
