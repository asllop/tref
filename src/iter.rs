
    // Simple Iterator

    pub struct TreeIter<'a, 'b, T: crate::tree::NodeContent> {
        tree: &'b crate::tree::TreeModel<'a, T>,
        position: usize
    }

    impl<'a, 'b, T: crate::tree::NodeContent> TreeIter<'a, 'b, T> {
        pub fn new(tree: &'b crate::tree::TreeModel<'a, T>) -> Self {
            Self {
                tree,
                position: 0
            }
        }
    }

    impl<'a, 'b, T: crate::tree::NodeContent> Iterator for TreeIter<'a, 'b, T> {
        type Item = &'a crate::tree::TreeNode<T>;
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

    // BFS Iterator

    pub struct BfsIter<'a, 'b, T: crate::tree::NodeContent> {
        tree: &'b crate::tree::TreeModel<'a, T>,
        position: usize,
        sub_position: usize
    }

    impl<'a, 'b, T: crate::tree::NodeContent> BfsIter<'a, 'b, T> {
        pub fn new(tree: &'b crate::tree::TreeModel<'a, T>) -> Self {
            Self {
                tree,
                position: 0,
                sub_position: 0
            }
        }
    }

    impl<'a, 'b, T: crate::tree::NodeContent> Iterator for BfsIter<'a, 'b, T> {
        type Item = &'a crate::tree::TreeNode<T>;
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

    // Inverse BSF Iterator

    pub struct InvBfsIter<'a, 'b, T: crate::tree::NodeContent> {
        tree: &'b crate::tree::TreeModel<'a, T>,
        position: usize,
        sub_position: usize
    }

    impl<'a, 'b, T: crate::tree::NodeContent> InvBfsIter<'a, 'b, T> {
        pub fn new(tree: &'b crate::tree::TreeModel<'a, T>) -> Self {
            Self {
                tree,
                position: tree.level_ref.len() - 1,
                sub_position: 0
            }
        }
    }

    impl<'a, 'b, T: crate::tree::NodeContent> Iterator for InvBfsIter<'a, 'b, T> {
        type Item = &'a crate::tree::TreeNode<T>;
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

    // NOTE: due to TREF structure, iter() and pre_dfs_iter() are equivalent.

    //TODO: DFS iterators: in-order and post-order.

    pub struct DfsIter<'a, 'b, T: crate::tree::NodeContent> {
        tree: &'b crate::tree::TreeModel<'a, T>,
        pila: Vec<u32>,
        next: u32,
        finished: bool,
        inverse: bool
    }

    impl<'a, 'b, T: crate::tree::NodeContent> DfsIter<'a, 'b, T> {
        pub fn new(tree: &'b crate::tree::TreeModel<'a, T>, inverse: bool) -> Self {
            Self {
                tree,
                pila: vec!(),
                next: 0,
                finished: false,
                inverse
            }
        }
    }

    impl<'a, 'b, T: crate::tree::NodeContent> Iterator for DfsIter<'a, 'b, T> {
        type Item = &'a crate::tree::TreeNode<T>;
        //1. use current node
        //2. put all children of current node in inverse order in stack
        //3. get from stack next node to visit
        //4. goto 1
        fn next(&mut self) -> Option<Self::Item> {
            if self.finished {
                return None;
            }
            // Get current node
            if let Some(node) = self.tree.tree_ref.nodes.get(self.next as usize) {
                // Put in the stack all children of current node
                if self.inverse {
                    for child in node.children.iter() {
                        self.pila.push(*child);
                    }
                }
                else {
                    for child in node.children.iter().rev() {
                        self.pila.push(*child);
                    }
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