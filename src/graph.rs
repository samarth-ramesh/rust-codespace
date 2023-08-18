use std::rc::Rc;
use crate::graph::NodeType::Principal;

pub enum RelationType {
    Has,
    Is
}
pub struct Relation {
    relation_type: RelationType,
    relation: *mut Rc<Node>
}

pub enum NodeType {
    Principal,
    Target,
    Group
}
pub struct Node {
    node_type: NodeType,
    relations: Vec<Relation>,
    value: String
}

pub struct Graph {
    v: Vec<Rc<Node>>
}

impl Graph {
    pub fn new() -> Self {
        Self { v: vec![] }
    }

    pub fn add_principal(&mut self, value: String) {
        let r = Rc::new(Node::new(Principal, value));
        self.v.push(r)
    }
}

impl Node {
    pub fn new(node_type: NodeType, value: String) -> Self {
        Self { node_type, relations: vec![], value }
    }
}


