mod graph;

fn main() {
    let mut g = graph::Graph::new();
    let s = String::from("hi");
    g.add_principal(s);
}
