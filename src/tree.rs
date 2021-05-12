use crate::stack;
use crate::tree;
use crate::iter;
use crate::forest::Forest;

#[derive(Debug)]
pub struct Tree<T: NodeContent> {
    pub nodes: Vec<TreeNode<T>>
}

impl<T: NodeContent> Tree<T> {
    pub fn new() -> Self {
        Self {
            nodes: vec!()
        }
    }

    pub fn add_root_node(&mut self, content: &String) -> bool {
        if let Some(n) = TreeNode::new_root(content) {
            self.nodes.push(n);
            true
        }
        else {
            false
        }
    }

    pub fn add_node(&mut self, content: &String, level: u32, parent_node_ref: &stack::NodeStackContent) -> u32 {
        if let Some(n) = TreeNode::new(&content, level, Some(parent_node_ref.tree_position)) {
            self.nodes.push(n);
            self.last_pos()
        }
        else {
            0
        }
    }

    pub fn last_pos(&self) -> u32 {
        self.nodes.len() as u32 - 1
    }

    pub fn get_mut_node(&mut self, parent_node_ref: &stack::NodeStackContent) -> Option<&mut tree::TreeNode<T>> {
        self.nodes.get_mut(parent_node_ref.tree_position as usize)
    }
}

#[derive(Debug)]
pub struct TreeLevel {
    pub level: u32,
    pub node_positions: Vec<u32>
}

pub trait NodeContent {
    fn new(content: String) -> Option<Self> where Self: Sized;
    fn get_content(&self) -> &String;
}

#[derive(Debug)]
pub struct SimpleNode {
    content: String
}

impl NodeContent for SimpleNode {
    fn new(content: String) -> Option<Self> {
        Some(Self { content })
    }

    fn get_content(&self) -> &String {
        &self.content
    }
}

#[derive(Debug)]
pub struct TreeNode<T: NodeContent> {
    pub content: T,
    pub level: u32,
    pub parent_position: Option<u32>,
    pub children: Vec<u32>
}

impl<T: NodeContent> TreeNode<T> {
    pub fn new(content: &String, level: u32, parent_position: Option<u32>) -> Option<Self> {
        if let Some(c) = T::new(String::from(content)) {
            Some(
                Self {
                    content: c,
                    level,
                    parent_position,
                    children: vec!()
                }
            )
        }
        else {
            None
        }
    }

    pub fn new_root(content: &String) -> Option<Self> {
        Self::new(content, 1, None)
    }

    pub fn add_child_node(&mut self, new_node_position: u32) {
        self.children.push(new_node_position);
    }
}

#[derive(Debug)]
pub struct TreeModel<'a, T: NodeContent> {
    pub tree_ref:  &'a Tree<T>,
    pub level_ref: Option<&'a Vec<TreeLevel>>
}

impl<'a, 'b, T: NodeContent> TreeModel<'a, T> {
    pub fn new(forest: &'a Forest<T>, tree_id: &String) -> Option<Self> {
        if let None = forest.trees.get(tree_id) {
            return None;
        }
        else if let Some(levels) = &forest.levels {
            if let None = levels.get(tree_id) {
                return None;
            }
        }
        
        if let Some(tree_ref) = forest.trees.get(tree_id) {
            let level_ref = if let Some(levels) = &forest.levels {
                levels.get(tree_id)
            }
            else {
                None
            };
            return Some(Self {
                tree_ref,
                level_ref
            });
        }
        
        None
    }

    pub fn iter(&'b self) -> iter::TreeIter<'a, 'b, T> {
        iter::TreeIter::new(self)
    }

    pub fn inv_iter(&'b self) -> iter::InvTreeIter<'a, 'b, T> {
        iter::InvTreeIter::new(self)
    }

    pub fn bfs_iter(&'b self) -> iter::BfsIter<'a, 'b, T> {
        iter::BfsIter::new(self)
    }

    pub fn inv_bfs_iter(&'b self) -> iter::InvBfsIter<'a, 'b, T> {
        iter::InvBfsIter::new(self)
    }

    pub fn inv_lev_bfs_iter(&'b self) -> iter::InvLevBfsIter<'a, 'b, T> {
        iter::InvLevBfsIter::new(self)
    }

    pub fn pre_dfs_iter(&'b self) -> iter::PreDfsIter<'a, 'b, T> {
        iter::PreDfsIter::new(self)
    }

    pub fn inv_pre_dfs_iter(&'b self) -> iter::InvPreDfsIter<'a, 'b, T> {
        iter::InvPreDfsIter::new(self)
    }

    pub fn post_dfs_iter(&'b self) -> iter::PostDfsIter<'a, 'b, T> {
        iter::PostDfsIter::new(self)
    }

    pub fn inv_post_dfs_iter(&'b self) -> iter::InvPostDfsIter<'a, 'b, T> {
        iter::InvPostDfsIter::new(self)
    }
}