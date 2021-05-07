
    // Simple Iterator

    pub struct TreeIter<'a, 'b> {
        tree: &'b crate::tree::TreeModel<'a>,
        position: usize
    }

    impl<'a, 'b> TreeIter<'a, 'b> {
        pub fn new(tree: &'b crate::tree::TreeModel<'a>) -> Self {
            Self {
                tree,
                position: 0
            }
        }
    }

    impl<'a, 'b> Iterator for TreeIter<'a, 'b> {
        type Item = &'a crate::tree::TreeNode;
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

    pub struct BfsIter<'a, 'b> {
        tree: &'b crate::tree::TreeModel<'a>,
        position: usize,
        sub_position: usize
    }

    impl<'a, 'b> BfsIter<'a, 'b> {
        pub fn new(tree: &'b crate::tree::TreeModel<'a>) -> Self {
            Self {
                tree,
                position: 0,
                sub_position: 0
            }
        }
    }

    impl<'a, 'b> Iterator for BfsIter<'a, 'b> {
        type Item = &'a crate::tree::TreeNode;
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

    pub struct InvBfsIter<'a, 'b> {
        tree: &'b crate::tree::TreeModel<'a>,
        position: usize,
        sub_position: usize
    }

    impl<'a, 'b> InvBfsIter<'a, 'b> {
        pub fn new(tree: &'b crate::tree::TreeModel<'a>) -> Self {
            Self {
                tree,
                position: tree.level_ref.len() - 1,
                sub_position: 0
            }
        }
    }

    impl<'a, 'b> Iterator for InvBfsIter<'a, 'b> {
        type Item = &'a crate::tree::TreeNode;
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

    //TODO: DFS iterators: in-order, pre-order and post-order.