#[derive(Debug)]
pub struct Tree {
    pub nodes: Vec<TreeNode>
}

impl Tree {
    pub fn new(content: &String) -> Self {
        let mut tree = Self {
            nodes: vec!()
        };
        tree.add_root_node(content);
        tree
    }

    pub fn add_root_node(&mut self, content: &String) {
        self.nodes.push(TreeNode::new_root(content));
    }

    pub fn add_node(&mut self, content: &String, level: u32, parent_node_ref: &crate::stack::NodeStackContent) -> u32 {
        self.nodes.push(TreeNode::new(&content, level, Some(parent_node_ref.tree_position)));
        self.last_pos()
    }

    pub fn last_pos(&self) -> u32 {
        self.nodes.len() as u32 - 1
    }

    pub fn get_mut_node(&mut self, parent_node_ref: &crate::stack::NodeStackContent) -> Option<&mut crate::tree::TreeNode> {
        self.nodes.get_mut(parent_node_ref.tree_position as usize)
    }
}

#[derive(Debug)]
pub struct TreeLevel {
    pub level: u32,
    pub node_positions: Vec<u32>
}

#[derive(Debug)]
pub struct TreeNode {
    pub content: String,
    pub level: u32,
    pub parent_position: Option<u32>,
    pub children: Vec<u32>
}

impl TreeNode {
    pub fn new(content: &String, level: u32, parent_position: Option<u32>) -> Self {
        Self {
            content: String::from(content),
            level,
            parent_position,
            children: vec!()
        }
    }

    pub fn new_root(content: &String) -> Self {
        Self::new(content, 1, None)
    }

    pub fn add_child_node(&mut self, new_node_position: u32) {
        self.children.push(new_node_position);
    }
}

#[derive(Debug)]
pub struct TreeModel<'a> {
    pub tree_ref:  &'a Tree,
    pub level_ref: &'a Vec<TreeLevel>
}

impl<'a, 'b> TreeModel<'a> {
    pub fn new(forest: &'a crate::Forest, tree_id: &String) -> Option<Self> {
        if let None = forest.trees.get(tree_id) {
            return None;
        }
        else if let None = forest.levels.get(tree_id) {
            return None;
        }
        else {
            if let Some(tree_ref) = forest.trees.get(tree_id) {
                if let Some(level_ref) = forest.levels.get(tree_id) {
                    return Some(Self {
                        tree_ref,
                        level_ref
                    });
                }
            }
        }
        None
    }

    pub fn bfs_iter(&'b self) -> crate::iter::BfsIter<'a, 'b> {
        crate::iter::BfsIter::new(self)
    }

    pub fn inv_bfs_iter(&'b self) -> crate::iter::InvBfsIter<'a, 'b> {
        crate::iter::InvBfsIter::new(self)
    }
}