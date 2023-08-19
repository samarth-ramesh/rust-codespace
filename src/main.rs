use crate::graph::RelationType::{Belongs, Is};

mod graph;

fn main() {
    let mut g = graph::Graph::new();
    let s = String::from("hi");
    g.add_principal(s.clone());
    g.print_principals();
    let item_idx = g.get_principal(s.clone()).expect("Error");
    g.create_group(s, item_idx);
}
