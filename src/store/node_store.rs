use crate::ast::Node;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

pub struct NodeStore {
    node_list: Vec<Node>,
}

impl NodeStore {
    pub fn new() -> Self {
        Self {
            node_list: Vec::new(),
        }
    }

    pub fn add(&mut self, node: Node) -> NodeId {
        let id = NodeId(self.node_list.len());
        self.node_list.push(node);
        return id;
    }

    pub fn get(&self, node_id: NodeId) -> Option<&Node> {
        if node_id.0 >= self.node_list.len() {
            None
        } else {
            Some(&self.node_list[node_id.0])
        }
    }

    pub fn debug(&self) {
        println!("debug: NodeStore\n");
        for (pos, node) in self.node_list.iter().enumerate() {
            println!("       {}: {}", pos, node.to_string());
        }
        println!();
    }
}
