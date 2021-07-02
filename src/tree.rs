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

    pub fn add_node(&mut self, content: &String, level: u32, parent_node_ref: &stack::NodeStackContent, parent_children_pos: Option<u32>) -> u32 {
        if let Some(n) = TreeNode::new(&content, level, Some(parent_node_ref.tree_position), parent_children_pos) {
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

/// Trait to define structs that model a node content.
pub trait NodeContent {
    /// Constructor.
    /// 
    /// # Aeguments
    /// 
    /// * `content` - Node content.
    /// 
    /// # Return
    /// 
    /// * An [`Option`] with the node content.
    /// 
    fn new(content: String) -> Option<Self> where Self: Sized;
    /// Get the raw node content.
    /// 
    /// # Return
    /// 
    /// * Node content.
    ///
    fn get_content(&self) -> &String;
}

/// Default [`NodeContent`] struct.
/// 
/// It simply holds the content as is, without parsing or modifying it.
#[derive(Debug)]
pub struct SimpleNode {
    /// Node content.
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

/// Struct that contains a tree node.
#[derive(Debug)]
pub struct TreeNode<T: NodeContent> {
    /// Raw node content.
    pub content: T,
    /// Nodel level.
    pub level: u32,
    /// Parent node index in the tree array.
    pub parent_position: Option<u32>,
    /// Index of current node in the parent [`children`][`TreeNode::children`] array.
    pub parent_children_pos: Option<u32>,
    /// Array that contains indexes of of children nodes.
    pub children: Vec<u32>
}

impl<T: NodeContent> TreeNode<T> {
    pub fn new(content: &String, level: u32, parent_position: Option<u32>, parent_children_pos: Option<u32>) -> Option<Self> {
        if let Some(c) = T::new(String::from(content)) {
            Some(
                Self {
                    content: c,
                    level,
                    parent_position,
                    parent_children_pos,
                    children: vec!()
                }
            )
        }
        else {
            None
        }
    }

    pub fn new_root(content: &String) -> Option<Self> {
        Self::new(content, 1, None, None)
    }

    pub fn add_child_node(&mut self, new_node_position: u32) {
        self.children.push(new_node_position);
    }
}

/// Struct that represents a tree.
#[derive(Debug)]
pub struct TreeModel<'a, T: NodeContent> {
    /// Reference to the tree structure, contained in the [`Forest`].
    pub tree_ref:  &'a Tree<T>,
    /// Reference to the tree levels structure, contained in the [`Forest`].
    pub level_ref: Option<&'a Vec<TreeLevel>>
}

impl<'a, 'b, T: NodeContent> TreeModel<'a, T> {
    /// Create a new tree model.
    /// 
    /// # Arguments
    /// 
    /// * `forest` - A [`Forest`] structure
    /// * `tree_id` - ID of the tree.
    ///
    /// # Return
    /// 
    /// * An [`Option`] with the tree model.
    /// 
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

    /// Create an iterator using simple traversal.
    /// 
    /// It traverses the node array from the first position to the last, ignoring the tree structure.
    /// 
    /// The iterator returns instances of [`TreeNode`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// let forest = ... // Obtain a Forest structure
    /// if let Some(tree_model) = forest.tree(&String::from("my_tree")) {
    ///     for n in tree_model.iter() {
    ///         println!("{}", n.content.get_content());
    ///     }
    /// }
    /// ```
    pub fn iter(&'b self) -> iter::TreeIter<'a, 'b, T> {
        iter::TreeIter::new(self)
    }

    /// Create an iterator using simple inverse traversal.
    /// 
    /// It traverses the node array from the last position to the first, ignoring the tree structure.
    /// 
    /// # Examples
    /// 
    /// Check [`iter()`][`TreeModel::iter()`], all iterators work the same way.
    /// 
    pub fn inv_iter(&'b self) -> iter::InvTreeIter<'a, 'b, T> {
        iter::InvTreeIter::new(self)
    }

    /// Create an iterator using the BFS algorithm.
    /// 
    /// # Examples
    /// 
    /// Check [`iter()`][`TreeModel::iter()`], all iterators work the same way.
    /// 
    pub fn bfs_iter(&'b self) -> iter::BfsIterSwitch<'a, 'b, T> {
        iter::BfsIterSwitch::new(self)
    }

    /// Create an iterator using the Inverse BFS algorithm.
    /// 
    /// # Examples
    /// 
    /// Check [`iter()`][`TreeModel::iter()`], all iterators work the same way.
    /// 
    pub fn inv_bfs_iter(&'b self) -> iter::InvBfsIterSwitch<'a, 'b, T> {
        iter::InvBfsIterSwitch::new(self)
    }

    /// Create an iterator using the Inverse BFS algorithm, using the [`levels`][`Forest::levels`] structure.
    /// 
    /// # Examples
    /// 
    /// Check [`iter()`][`TreeModel::iter()`], all iterators work the same way.
    /// 
    pub fn inv_lev_bfs_iter(&'b self) -> iter::level_iters::InvLevBfsIter<'a, 'b, T> {
        iter::level_iters::InvLevBfsIter::new(self)
    }

    /// Create an iterator using the DFS algorithm.
    /// 
    /// # Examples
    /// 
    /// Check [`iter()`][`TreeModel::iter()`], all iterators work the same way.
    /// 
    pub fn pre_dfs_iter(&'b self) -> iter::PreDfsIter<'a, 'b, T> {
        iter::PreDfsIter::new(self)
    }

    /// Create an iterator using the Pre-DFS algorithm.
    /// 
    /// # Examples
    /// 
    /// Check [`iter()`][`TreeModel::iter()`], all iterators work the same way.
    /// 
    pub fn inv_pre_dfs_iter(&'b self) -> iter::InvPreDfsIter<'a, 'b, T> {
        iter::InvPreDfsIter::new(self)
    }

    /// Create an iterator using the Post-DFS algorithm.
    /// 
    /// # Examples
    /// 
    /// Check [`iter()`][`TreeModel::iter()`], all iterators work the same way.
    /// 
    pub fn post_dfs_iter(&'b self) -> iter::PostDfsIter<'a, 'b, T> {
        iter::PostDfsIter::new(self)
    }

    /// Create an iterator using the Inverse Post-DFS algorithm.
    /// 
    /// # Examples
    /// 
    /// Check [`iter()`][`TreeModel::iter()`], all iterators work the same way.
    /// 
    pub fn inv_post_dfs_iter(&'b self) -> iter::InvPostDfsIter<'a, 'b, T> {
        iter::InvPostDfsIter::new(self)
    }
}