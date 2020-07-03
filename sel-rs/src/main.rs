use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
}

impl Node {
    pub fn new_with_name(name: &str) -> Node {
        Node {
            name: name.to_string(),
            left: None,
            right: None,
        }
    }

    pub fn is_isomorphic(&self, node: &Node) -> bool {
        if self.name != node.name {
            false
        } else {
            let leq = match (&self.left, node.left.as_ref()) {
                (None, None) => true,
                (Some(a), Some(b)) => a.is_isomorphic(&b),
                _ => false,
            };
            let req = match (&self.right, node.right.as_ref()) {
                (None, None) => true,
                (Some(a), Some(b)) => a.is_isomorphic(&b),
                _ => false,
            };
            leq && req
        }
    }
}

fn program() -> Node {
    let mut add = Node::new_with_name("add");
    let a = Node::new_with_name("a");
    let b = Node::new_with_name("b");
    add.left = Some(Rc::new(a));
    add.right = Some(Rc::new(b));
    add
}

fn pattern() -> Node {
    let mut add = Node::new_with_name("add");
    let a = Node::new_with_name("a");
    let b = Node::new_with_name("b");
    add.left = Some(Rc::new(a));
    add.right = Some(Rc::new(b));
    add
}

fn main() {
    let prog = program();
    let pat = pattern();
    println!("match:{}", prog.is_isomorphic(&pat));
}
