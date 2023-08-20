pub mod graph {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

    use queues::{IsQueue, Queue};

    use crate::graph::graph::Status::{DuplicateRelation, NotFound, Ok};

    extern crate queues;

    pub enum Status {
        Ok,
        NotFound,
        DuplicateRelation,
    }

    #[derive(Hash, Clone, Copy)]
    pub enum RelationType {
        Is,
        Belongs,
        Has,
        Implies,
    }

    #[derive(Hash, Eq, PartialEq)]
    pub enum NodeType {
        Principal,
        // has indegree == 0
        Permission,
        // has outdegree == 0
        Group, // can have any indegree or outdegree
    }

    // used for adj. list calculations
    #[derive(Hash, Eq, PartialEq)]
    pub struct Node {
        uid: String,
        node_type: NodeType,
    }

    pub struct Graph {
        adj_matrix: Box<HashMap<Node, HashMap<Node, RelationType>>>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                adj_matrix: Box::from(HashMap::new())
            }
        }

        pub fn get_hash(node: *const Node) -> u64 {
            let mut hasher = DefaultHasher::new();
            node.hash(&mut hasher);
            return hasher.finish();
        }

        pub fn insert_generic(&mut self, uid: String, node_type: NodeType) {
            let n = Node::new(uid, node_type);
            self.adj_matrix.insert(n, HashMap::new());
        }

        pub fn add_link(&mut self, uid1: String, uid2: String, node_type1: NodeType, node_type2: NodeType, relation_type: RelationType) -> Status {
            let n1 = Node::new(uid1, node_type1);
            let n2 = Node::new(uid2, node_type2);

            {
                // checking phase
                let r1 = self.adj_matrix.get(&n1);
                if r1.is_none() {
                    return NotFound;
                }
                let r1 = r1.unwrap();

                // sanity check. Should never usually be the case
                if r1.contains_key(&n2){
                    return DuplicateRelation;
                }

                let does_dest_exist = self.adj_matrix.contains_key(&n2);
                if !does_dest_exist {
                    return NotFound;
                }
            }


            {
                // time to yeet stuff in
                let r1 = self.adj_matrix.get_mut(&n1).unwrap();
                r1.insert(n2, relation_type);
            }

            return Ok;
        }
        pub fn check_link(&self, uid1: String, uid2: String, node_type1: NodeType, node_type2: NodeType) -> Result<bool, Status> {
            let n1 = Node::new(uid1, node_type1);
            let n2 = Node::new(uid2, node_type2);
            let h2 = Self::get_hash(&n2);

            // checking phase
            let r1 = self.adj_matrix.get(&n1);
            let does_dest_exist = self.adj_matrix.contains_key(&n2);
            if r1.is_none() || !does_dest_exist {
                return Err(NotFound);
            }

            let mut q: Queue<&Node> = Queue::new();
            let mut visited: HashMap<u64, bool> = HashMap::new();
            q.add(&n1).unwrap();

            while q.size() > 0 {
                let cur = q.remove().unwrap();
                let cur_hash = Self::get_hash(cur);
                if visited.contains_key(&cur_hash) {
                    continue;
                } else {
                    visited.insert(cur_hash, true);
                }

                if cur_hash.eq(&h2) {
                    return Result::Ok(true);
                }

                let cur_children = self.adj_matrix.get(cur).unwrap();
                cur_children.iter().for_each(|(key, _)| {
                    q.add(key).unwrap();
                })
            }

            return Result::Ok(false);
        }
    }

    impl Node {
        pub fn new(uid: String, node_type: NodeType) -> Self {
            Self { uid, node_type }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::graph::Graph;
    use crate::graph::graph::NodeType::{Permission, Principal};
    use crate::graph::graph::RelationType::Has;

    #[test]
    fn test_create_link() {
        let mut g = Graph::new();
        g.insert_generic(String::from("1"), Principal);
        g.insert_generic(String::from("2"), Permission);
        g.add_link(String::from("1"), String::from("2"), Principal, Permission, Has);
        assert!(g.check_link(String::from("1"), String::from("2"), Principal, Permission).is_ok_and(|x| {x}));
    }
}