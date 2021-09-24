#[derive(Debug)]
pub struct NodeStack {
    buffer: Vec<NodeStackContent>
}

#[derive(Debug)]
pub struct NodeStackContent {
    level: usize,
    pub tree_position: usize
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

    pub fn push_new(&mut self, level: usize, tree_position: usize) {
        self.buffer.push(NodeStackContent::new(level, tree_position));
    }

    pub fn pop(&mut self) -> Option<NodeStackContent> {
        self.buffer.pop()
    }

    pub fn pop_parent(&mut self, level: usize) -> Option<NodeStackContent> {
        // Obtain data from stack until we get one node with a level lower than "level"
        while let Some(n_node) = self.pop() {
            if n_node.level < level {
                return Some(n_node);
            }
        }
        None
    }

    pub fn flush(&mut self) {
        self.buffer.truncate(0);
    }

    pub fn top(&mut self) -> Option<&NodeStackContent> {
        self.buffer.last()
    }
}

impl NodeStackContent {
    pub fn new(level: usize, tree_position: usize) -> Self {
        Self {
            level,
            tree_position
        }
    }
}