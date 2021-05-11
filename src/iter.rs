use crate::tree;

// Simple Iterator

pub struct TreeIter<'a, 'b, T: tree::NodeContent> {
    tree: &'b tree::TreeModel<'a, T>,
    position: usize
}

impl<'a, 'b, T: tree::NodeContent> TreeIter<'a, 'b, T> {
    pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
        Self {
            tree,
            position: 0
        }
    }
}

impl<'a, 'b, T: tree::NodeContent> Iterator for TreeIter<'a, 'b, T> {
    type Item = &'a tree::TreeNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.tree.tree_ref.nodes.get(self.position) {
            Some(node) => {
                self.position += 1;
                Some(node)
            },
            None => None
        }
    }
}

// Inverse Simple Iterator

pub struct InvTreeIter<'a, 'b, T: tree::NodeContent> {
    tree: &'b tree::TreeModel<'a, T>,
    position: usize,
    finished: bool
}

impl<'a, 'b, T: tree::NodeContent> InvTreeIter<'a, 'b, T> {
    pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
        if tree.tree_ref.nodes.len() > 0 {
            Self {
                tree,
                position: tree.tree_ref.nodes.len() - 1,
                finished: false
            }
        }
        else {
            Self {
                tree,
                position: 0,
                finished: true
            }
        }
    }
}

impl<'a, 'b, T: tree::NodeContent> Iterator for InvTreeIter<'a, 'b, T> {
    type Item = &'a tree::TreeNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        match &self.tree.tree_ref.nodes.get(self.position) {
            Some(node) => {
                if self.position > 0 {
                    self.position -= 1;
                }
                else {
                    self.finished = true;
                }
                Some(node)
            },
            None => None
        }
    }
}

//TODO: implement BFS using a queue and make "levels" structure optional (or even remove it)

// BFS Iterator

pub struct BfsIter<'a, 'b, T: tree::NodeContent> {
    tree: &'b tree::TreeModel<'a, T>,
    position: usize,
    sub_position: usize
}

impl<'a, 'b, T: tree::NodeContent> BfsIter<'a, 'b, T> {
    pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
        Self {
            tree,
            position: 0,
            sub_position: 0
        }
    }
}

impl<'a, 'b, T: tree::NodeContent> Iterator for BfsIter<'a, 'b, T> {
    type Item = &'a tree::TreeNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tree_level) = self.tree.level_ref.get(self.position) {
            if let Some(node_position) = tree_level.node_positions.get(self.sub_position) {
                self.sub_position += 1;
                return self.tree.tree_ref.nodes.get(*node_position as usize);
            }
            else {
                self.position += 1;
                self.sub_position = 0;                    
                return self.next();
            }
        }
        None
    }
}

// Inverse BFS Iterator

pub struct InvBfsIter<'a, 'b, T: tree::NodeContent> {
    tree: &'b tree::TreeModel<'a, T>,
    position: usize,
    sub_position: usize
}

impl<'a, 'b, T: tree::NodeContent> InvBfsIter<'a, 'b, T> {
    pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
        Self {
            tree,
            position: tree.level_ref.len() - 1,
            sub_position: 0
        }
    }
}

impl<'a, 'b, T: tree::NodeContent> Iterator for InvBfsIter<'a, 'b, T> {
    type Item = &'a tree::TreeNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tree_level) = self.tree.level_ref.get(self.position) {
            if let Some(node_position) = tree_level.node_positions.get(self.sub_position) {
                self.sub_position += 1;
                return self.tree.tree_ref.nodes.get(*node_position as usize);
            }
            else {
                if self.position == 0 {
                    return None;
                }
                self.position -= 1;
                self.sub_position = 0;                    
                return self.next();
            }
        }
        None
    }
}

// Pre-Order DFS

pub struct PreDfsIter<'a, 'b, T: tree::NodeContent> {
    tree: &'b tree::TreeModel<'a, T>,
    pila: Vec<u32>,
    next: u32,
    finished: bool
}

impl<'a, 'b, T: tree::NodeContent> PreDfsIter<'a, 'b, T> {
    pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
        Self {
            tree,
            pila: vec!(),
            next: 0,
            finished: false
        }
    }
}

impl<'a, 'b, T: tree::NodeContent> Iterator for PreDfsIter<'a, 'b, T> {
    type Item = &'a tree::TreeNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        // Get current node
        if let Some(node) = self.tree.tree_ref.nodes.get(self.next as usize) {
            // Put in the stack all children of current node
            for child in node.children.iter().rev() {
                self.pila.push(*child);
            }
            // Get next node from stack.
            if let Some(next_node_index) = self.pila.pop() {
                self.next = next_node_index;
            }
            else {
                // If nothing in stack, end
                self.finished = true;
            }
            // Return current node
            Some(node)
        }
        else {
            None
        }

    }
}

// Inverse Pre-Order DFS

pub struct InvPreDfsIter<'a, 'b, T: tree::NodeContent> {
    tree: &'b tree::TreeModel<'a, T>,
    pila: Vec<u32>,
    next: u32,
    finished: bool
}

impl<'a, 'b, T: tree::NodeContent> InvPreDfsIter<'a, 'b, T> {
    pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
        Self {
            tree,
            pila: vec!(),
            next: 0,
            finished: false
        }
    }
}

impl<'a, 'b, T: tree::NodeContent> Iterator for InvPreDfsIter<'a, 'b, T> {
    type Item = &'a tree::TreeNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        // Get current node
        if let Some(node) = self.tree.tree_ref.nodes.get(self.next as usize) {
            // Put in the stack all children of current node
            for child in node.children.iter() {
                self.pila.push(*child);
            }
            // Get next node from stack.
            if let Some(next_node_index) = self.pila.pop() {
                self.next = next_node_index;
            }
            else {
                // If nothing in stack, end
                self.finished = true;
            }
            // Return current node
            Some(node)
        }
        else {
            None
        }

    }
}

// Post-Order DFS

pub struct PostDfsIter<'a, 'b, T: tree::NodeContent> {
    tree: &'b tree::TreeModel<'a, T>,
    pila: Vec<(u32, bool)>
}

impl<'a, 'b, T: tree::NodeContent> PostDfsIter<'a, 'b, T> {
    pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
        Self {
            tree,
            pila: vec!((0, true))
        }
    }
}

impl<'a, 'b, T: tree::NodeContent> Iterator for PostDfsIter<'a, 'b, T> {
    type Item = &'a tree::TreeNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        // Get current node
        if let Some(next_node_tuple) = self.pila.pop() {
            // found something in the stack
            let next = next_node_tuple.0;
            let push_children = next_node_tuple.1;
            // get node from tree
            if let Some(node) = self.tree.tree_ref.nodes.get(next as usize) {
                if !push_children {
                    return Some(node);
                }
                // it has children, put in stack
                if node.children.len() > 0 {
                    self.pila.push((next, false));
                    for child in node.children.iter().rev() {
                        self.pila.push((*child, true));
                    }
                    // Keep trying until we find a node we can return
                    return self.next();
                }
                // if no children, return this one
                else {
                    return Some(node);
                }
            }
            else {
                // Bad thing, a broken index
                return None;
            }
        }

        None
    }
}

// Inverse Post-Order

pub struct InvPostDfsIter<'a, 'b, T: tree::NodeContent> {
    tree: &'b tree::TreeModel<'a, T>,
    pila: Vec<(u32, bool)>
}

impl<'a, 'b, T: tree::NodeContent> InvPostDfsIter<'a, 'b, T> {
    pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
        Self {
            tree,
            pila: vec!((0, true))
        }
    }
}

impl<'a, 'b, T: tree::NodeContent> Iterator for InvPostDfsIter<'a, 'b, T> {
    type Item = &'a tree::TreeNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        // Get current node
        if let Some(next_node_tuple) = self.pila.pop() {
            // found something in the stack
            let next = next_node_tuple.0;
            let push_children = next_node_tuple.1;
            // get node from tree
            if let Some(node) = self.tree.tree_ref.nodes.get(next as usize) {
                if !push_children {
                    return Some(node);
                }
                // it has children, put in stack
                if node.children.len() > 0 {
                    self.pila.push((next, false));
                    for child in node.children.iter() {
                        self.pila.push((*child, true));
                    }
                    // Keep trying until we find a node we can return
                    return self.next();
                }
                // if no children, return this one
                else {
                    return Some(node);
                }
            }
            else {
                // Bad thing, a broken index
                return None;
            }
        }

        None
    }
}