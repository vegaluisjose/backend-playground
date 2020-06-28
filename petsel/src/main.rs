use petgraph::Graph;
use petgraph::dot::{Dot, Config};
use std::fs::File;
use std::io::Write;

fn main() {
    let mut dag : Graph<&str, &str, petgraph::Directed> = Graph::new();
    let a = dag.add_node("a");
    let b = dag.add_node("b");
    let c = dag.add_node("c");
    let d = dag.add_node("d");
    let e = dag.add_node("e");
    dag.add_edge(a, b, "");
    dag.add_edge(a, c, "");
    dag.add_edge(b, d, "");
    dag.add_edge(b, e, "");
    println!("{:?}", Dot::with_config(&dag, &[Config::EdgeNoLabel]));
    let mut f = File::create("example.dot").unwrap();
    let output = format!("{}", Dot::with_config(&dag, &[Config::EdgeNoLabel]));
    f.write_all(&output.as_bytes()).expect("could not write file");
}
