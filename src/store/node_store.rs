use crate::ast::Node;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

pub struct NodeStore {
    node_list: Vec<Option<Node>>,
}

impl NodeStore {
    pub fn new() -> Self {
        Self {
            node_list: Vec::new(),
        }
    }

    pub fn add(&mut self, node: Node) -> NodeId {
        let id = NodeId(self.node_list.len());
        self.node_list.push(Some(node));
        return id;
    }

    pub fn get(&self, node_id: NodeId) -> Option<&Node> {
        self.node_list.get(node_id.0)?.as_ref()
    }

    pub fn get_mut(&mut self, node_id: NodeId) -> Option<&mut Node> {
        self.node_list.get_mut(node_id.0)?.as_mut()
    }

    pub fn take(&mut self, node_id: NodeId) -> Option<Node> {
        self.node_list.get_mut(node_id.0)?.take()
    }

    pub fn put_back(&mut self, node_id: NodeId, node: Node) -> bool {
        match self.node_list.get_mut(node_id.0) {
            Some(slot @ None) => {
                *slot = Some(node);
                true
            }
            _ => false,
        }
    }

    pub fn debug(&self) {
        println!("debug: NodeStore\n");
        for (pos, node) in self.node_list.iter().enumerate() {
            if let Some(node) = node {
                println!("       {}: {}", pos, node.to_string());
            }
        }
        println!();
    }
}
