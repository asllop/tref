#[derive(Debug)]
pub struct NodeStack {
    buffer: Vec<NodeStackContent>
}

#[derive(Debug)]
pub struct NodeStackContent {
    level: u32,
    pub tree_position: u32
}

impl NodeStack {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }

    pub fn push(&mut self, obj: NodeStackContent) {
        self.buffer.push(obj);
    }

    pub fn push_new(&mut self, level: u32, tree_position: u32) {
        self.buffer.push(crate::stack::NodeStackContent::new(level, tree_position));
    }

    pub fn pop(&mut self) -> Option<NodeStackContent> {
        self.buffer.pop()
    }

    pub fn pop_parent(&mut self, level: u32) -> Option<NodeStackContent> {
        // Obtain data from stack until we get one node with a level lower than "level"
        loop {
            let n = self.pop();
            if let Some(n_node) = n {
                if n_node.level < level {
                    return Some(n_node);
                }
            }
            else {
                return None;
            }
        }
    }

    pub fn flush(&mut self) {
        self.buffer.truncate(0);
    }

    pub fn top(&mut self) -> Option<&NodeStackContent> {
        self.buffer.last()
    }
}

impl NodeStackContent {
    pub fn new(level: u32, tree_position: u32) -> Self {
        Self {
            level,
            tree_position
        }
    }
}