use crate::ast::Node;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

#[derive(Debug, Clone)]
pub enum NodeType {
    DefineNode,
    ForNode,
    IfNode,
    IncludeNode,
    InterpolateNode,
    OnceNode,
    TextNode,
    RootNode,
    WithNode,
}

pub struct NodeStore {
    node_list: Vec<Option<Node>>,
    node_type_map: HashMap<NodeId, NodeType>,
    blacklist: HashSet<NodeId>,
}

impl NodeStore {
    pub fn new() -> Self {
        Self {
            node_list: Vec::new(),
            blacklist: HashSet::new(),
            node_type_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, node: Node, node_type: NodeType) -> NodeId {
        let id = NodeId(self.node_list.len());
        self.node_type_map.insert(id, node_type);
        self.node_list.push(Some(node));
        return id;
    }

    pub fn get(&self, node_id: NodeId) -> Option<&Node> {
        self.node_list.get(node_id.0)?.as_ref()
    }

    pub fn get_clone(&self, node_id: NodeId) -> Option<Node> {
        self.get(node_id).cloned()
    }

    pub fn get_mut(&mut self, node_id: NodeId) -> Option<&mut Node> {
        self.node_list.get_mut(node_id.0)?.as_mut()
    }

    pub fn get_type(&self, node_id: NodeId) -> Option<NodeType> {
        self.node_type_map.get(&node_id).cloned()
    }

    pub fn add_to_blacklist(&mut self, node_id: NodeId) {
        self.blacklist.insert(node_id);
    }

    pub fn is_blacklisted(&self, node_id: NodeId) -> bool {
        self.blacklist.contains(&node_id)
    }

    pub fn debug(&self) {
        println!("debug: NodeStore\n");
        for (pos, node) in self.node_list.iter().enumerate() {
            if let Some(node) = node {
                let node_type = self.node_type_map.get(&NodeId(pos)).unwrap();
                println!("       {}({:?}): {}", pos, node_type, node.to_string());
            }
        }
        println!();

        println!("       Blacklisted:\n       {:?}\n", self.blacklist);
    }
}
