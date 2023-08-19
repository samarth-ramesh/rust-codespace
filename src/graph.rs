extern crate queues;

use std::rc::Rc;
use crate::graph::NodeType::{Group, Principal};
use crate::graph::RelationType::{Belongs, Is};
use queues::{IsQueue, Queue};
#[derive(PartialEq)]
pub enum RelationType {
    Has,
    Is,
    Belongs,
}

pub struct Relation {
    relation_type: RelationType,
    relation: Box<Node>,
}

impl Relation {
    pub fn new(relation_type: RelationType, relation: Box<Node>) -> Self {
        Self { relation_type, relation }
    }
}
#[derive(Clone)]
pub enum NodeType {
    Principal,
    Target,
    Group,
}

pub struct Node {
    node_type: NodeType,
    relations: Vec<Relation>,
    value: String,
}

pub struct Graph {
    v: Vec<Box<Node>>,
}

impl Graph {
    pub fn new() -> Self {
        Self { v: vec![] }
    }

    pub fn add_principal(&mut self, value: String) {
        let r = Box::new(Node::new(Principal, value));
        self.v.push(r)
    }

    pub fn print_principals(&self) {
        for i in self.v.as_slice() {
            println!("{}", i.value)
        }
    }

    pub fn create_group(&mut self, value: String, idx: usize) {
        let r = Box::from(Node::new(Group, value));
        self.v[idx].relations.push(Relation::new(Belongs, r))
    }

    pub fn get_principal(&self, name: String) -> (Option<usize>) {
        for i in 0..self.v.len() {
            if self.v[i].value == name {
                return Option::from(i);
            }
        }
        return None
    }

    pub fn does_relation_exist(&self, start: usize, dest: String, dest_relation_type: RelationType) -> bool {
        let mut q: Queue<&Node> = Queue::new();
        q.add(self.v[start].as_ref()).expect("could not add");
        while q.size() > 0 {
            let cur = q.remove().expect("could not pop");
            for relation in cur.relations.as_slice() {
                if relation.relation_type == dest_relation_type && relation.relation.value == dest {
                    return  true;
                } else {
                    q.add(relation.relation.as_ref()).expect("could not add in BFS");
                }
            }
        }
        return false;
    }
}

impl Node {
    pub fn new(node_type: NodeType, value: String) -> Self {
        Self { node_type, relations: vec![], value }
    }
}

