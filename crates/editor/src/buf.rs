pub struct CodeBuffer {
    original: String,
    added: String,
    nodes: Vec<Node>,
}

pub struct Node {
    pub node_type: NodeType,
    pub start: usize,
    pub length: usize,
}

pub enum NodeType {
    Original,
    Added,
}

impl CodeBuffer {
    pub fn new(buf: String) -> Self {
        let len = buf.len();
        Self {
            original: buf,
            added: String::new(),
            nodes: vec![Node {
                node_type: NodeType::Original,
                start: 0,
                length: len,
            }],
        }
    }
}

impl ToString for CodeBuffer {
    fn to_string(&self) -> String {
        self.original.clone()
    }
}
