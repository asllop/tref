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
    type Item = (&'a tree::TreeNode<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position;
        match &self.tree.tree_ref.nodes.get(self.position) {
            Some(node) => {
                self.position += 1;
                Some((node, position))
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
    type Item = (&'a tree::TreeNode<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let position = self.position;
        match &self.tree.tree_ref.nodes.get(self.position) {
            Some(node) => {
                if self.position > 0 {
                    self.position -= 1;
                }
                else {
                    self.finished = true;
                }
                Some((node, position))
            },
            None => None
        }
    }
}

// BFS Iterator

pub struct BfsIter<'a, 'b, T: tree::NodeContent> {
    tree: &'b tree::TreeModel<'a, T>,
    // TODO: use std::collections::VecDeque instead of Vec
    cua: Vec<u32>,
    next: u32,
    finished: bool
}

impl<'a, 'b, T: tree::NodeContent> BfsIter<'a, 'b, T> {
    pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
        Self {
            tree,
            cua: vec!(),
            next: 0,
            finished: false
        }
    }
}

impl<'a, 'b, T: tree::NodeContent> Iterator for BfsIter<'a, 'b, T> {
    type Item = (&'a tree::TreeNode<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        // Get current node
        let position = self.next as usize;
        if let Some(node) = self.tree.tree_ref.nodes.get(position) {
            // Put in the queue all children of current node
            for child in node.children.iter() {
                self.cua.push(*child);
            }
            // Get next node from queue.
            if self.cua.len() > 0 {
                self.next = self.cua.remove(0);
            }
            else {
                // If nothing in thq queue, end
                self.finished = true;
            }
            // Return current node
            Some((node, position))
        }
        else {
            None
        }

    }
}

// Inverse BFS Iterator

pub struct InvBfsIter<'a, 'b, T: tree::NodeContent> {
    tree: &'b tree::TreeModel<'a, T>,
    // TODO: use std::collections::VecDeque instead of Vec
    cua: Vec<u32>,
    next: u32,
    finished: bool
}

impl<'a, 'b, T: tree::NodeContent> InvBfsIter<'a, 'b, T> {
    pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
        Self {
            tree,
            cua: vec!(),
            next: 0,
            finished: false
        }
    }
}

impl<'a, 'b, T: tree::NodeContent> Iterator for InvBfsIter<'a, 'b, T> {
    type Item = (&'a tree::TreeNode<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        // Get current node
        let position = self.next as usize;
        if let Some(node) = self.tree.tree_ref.nodes.get(position) {
            // Put in the queue all children of current node
            for child in node.children.iter().rev() {
                self.cua.push(*child);
            }
            // Get next node from queue.
            if self.cua.len() > 0 {
                self.next = self.cua.remove(0);
            }
            else {
                // If nothing in thq queue, end
                self.finished = true;
            }
            // Return current node
            Some((node, position))
        }
        else {
            None
        }

    }
}

// BFS Iterator Switch

pub struct BfsIterSwitch<'a, 'b, T: tree::NodeContent> {
    iter: Option<BfsIter<'a, 'b, T>>,
    level_iter: Option<level_iters::BfsIter<'a, 'b, T>>,
    levels: bool,
}

impl<'a, 'b, T: tree::NodeContent> BfsIterSwitch<'a, 'b, T> {
    pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
        let levels = if let Some(_) = tree.level_ref { true } else { false };
        Self {
            levels,
            iter: if levels { None } else { Some(BfsIter::new(tree)) },
            level_iter: if levels { Some(level_iters::BfsIter::new(tree)) } else { None }
        }
    }
}

impl<'a, 'b, T: tree::NodeContent> Iterator for BfsIterSwitch<'a, 'b, T> {
    type Item = (&'a tree::TreeNode<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.levels {
            return self.level_iter.as_mut().unwrap().next();
        }
        else {
            return self.iter.as_mut().unwrap().next();
        }
    }
}

// Inverse BFS Iterator Switch

pub struct InvBfsIterSwitch<'a, 'b, T: tree::NodeContent> {
    iter: Option<InvBfsIter<'a, 'b, T>>,
    level_iter: Option<level_iters::InvBfsIter<'a, 'b, T>>,
    levels: bool
}

impl<'a, 'b, T: tree::NodeContent> InvBfsIterSwitch<'a, 'b, T> {
    pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
        let levels = if let Some(_) = tree.level_ref { true } else { false };
        Self {
            levels,
            iter: if levels { None } else { Some(InvBfsIter::new(tree)) },
            level_iter: if levels { Some(level_iters::InvBfsIter::new(tree)) } else { None }
        }
    }
}

impl<'a, 'b, T: tree::NodeContent> Iterator for InvBfsIterSwitch<'a, 'b, T> {
    type Item = (&'a tree::TreeNode<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.levels {
            return self.level_iter.as_mut().unwrap().next();
        }
        else {
            return self.iter.as_mut().unwrap().next();
        }
    }
}

// Iterators using the levels structure
pub mod level_iters {
    use crate::tree;

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
        type Item = (&'a tree::TreeNode<T>, usize);
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(level_ref) = self.tree.level_ref {
                if let Some(tree_level) = level_ref.get(self.position) {
                    if let Some(node_position) = tree_level.node_positions.get(self.sub_position) {
                        self.sub_position += 1;
                        let position = *node_position as usize;
                        return match self.tree.tree_ref.nodes.get(position) {
                            Some(n) => Some((n, position)),
                            None => None
                        };
                    }
                    else {
                        self.position += 1;
                        self.sub_position = 0;                    
                        return self.next();
                    }
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
            let position = if let Some(level_ref) = tree.level_ref {
                level_ref.len() - 1
            }
            else {
                usize::MAX
            };
            Self {
                tree,
                position,
                sub_position: 0
            }
        }
    }

    impl<'a, 'b, T: tree::NodeContent> Iterator for InvBfsIter<'a, 'b, T> {
        type Item = (&'a tree::TreeNode<T>, usize);
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(level_ref) = self.tree.level_ref {
                if let Some(tree_level) = level_ref.get(self.position) {
                    if let Some(node_position) = tree_level.node_positions.get(self.sub_position) {
                        self.sub_position += 1;
                        let position = *node_position as usize;
                        return match self.tree.tree_ref.nodes.get(position) {
                            Some(n) => Some((n, position)),
                            None => None
                        };
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
            }
            None
        }
    }

    // Inverse Level BFS Iterator

    pub struct InvLevBfsIter<'a, 'b, T: tree::NodeContent> {
        tree: &'b tree::TreeModel<'a, T>,
        position: usize,
        sub_position: usize
    }

    impl<'a, 'b, T: tree::NodeContent> InvLevBfsIter<'a, 'b, T> {
        pub fn new(tree: &'b tree::TreeModel<'a, T>) -> Self {
            let position = if let Some(level_ref) = tree.level_ref {
                    level_ref.len() - 1
            }
            else {
                usize::MAX
            };
            Self {
                tree,
                position,
                sub_position: 0
            }
        }
    }

    impl<'a, 'b, T: tree::NodeContent> Iterator for InvLevBfsIter<'a, 'b, T> {
        type Item = (&'a tree::TreeNode<T>, usize);
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(level_ref) = self.tree.level_ref {
                if let Some(tree_level) = level_ref.get(self.position) {
                    if let Some(node_position) = tree_level.node_positions.get(self.sub_position) {
                        self.sub_position += 1;
                        let position = *node_position as usize;
                        return match self.tree.tree_ref.nodes.get(position) {
                            Some(n) => Some((n, position)),
                            None => None
                        };
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
            }
            None
        }
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
    type Item = (&'a tree::TreeNode<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        // Get current node
        let position = self.next as usize;
        if let Some(node) = self.tree.tree_ref.nodes.get(position) {
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
            Some((node, position))
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
    type Item = (&'a tree::TreeNode<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        // Get current node
        let position = self.next as usize;
        if let Some(node) = self.tree.tree_ref.nodes.get(position) {
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
            Some((node, position))
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
    type Item = (&'a tree::TreeNode<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        // Get current node
        if let Some(next_node_tuple) = self.pila.pop() {
            // found something in the stack
            let next = next_node_tuple.0;
            let push_children = next_node_tuple.1;
            // get node from tree
            let position = next as usize;
            if let Some(node) = self.tree.tree_ref.nodes.get(position) {
                // We already pushed children of this node. Return the node itself.
                if !push_children {
                    return Some((node, position));
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
                    return Some((node, position));
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
    type Item = (&'a tree::TreeNode<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        // Get current node
        if let Some(next_node_tuple) = self.pila.pop() {
            // found something in the stack
            let next = next_node_tuple.0;
            let push_children = next_node_tuple.1;
            // get node from tree
            let position = next as usize;
            if let Some(node) = self.tree.tree_ref.nodes.get(position) {
                if !push_children {
                    return Some((node, position));
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
                    return Some((node, position));
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