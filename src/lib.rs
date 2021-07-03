//! # TREF
//!
//! TREF is a plain text file format to describe trees in a human readable way.
//! 
//! With TREF a human can write a tree and understand the structure by having a quick look, because it is designed to be both easy to read for humans and easy to parse for machines.
//! 
//! Writing a tree in a file can be useful for many reasons: as a config file for an application, to store information that can be modified and read by an app and its user, to serialize tree-like memory structures, etc.
//! 
//! A simple TREF file looks like:
//! 
//! ```
//! # A simple tree.
//! [my_tree]
//! + root_nodess
//! + + child_1
//! + + + child_1_1
//! + + + child_1_2
//! + + child_2
//! + + + child_2_1
//! + + child_3
//! ```
//! File `simpletree.tref`
//! 
//! Check out the repo [README](https://github.com/asllop/tref/blob/master/README.md) for further details about the TREF format.
//! 
//! # Examples
//! 
//! To load this crate just use:
//! 
//! ```
//! use tref::*;
//! ```
//! 
//! Parse the `simpletree.tref` file and traverse `my_tree`:
//! 
//! ```
//! if let Ok(file) = File::open("simpletree.tref") {
//!     let forest: Forest<SimpleNode> = match Forest::build_levels(BufReader::new(file)) {
//!         Ok(f) => f,
//!         Err(m) => panic!("Could not parse TREF: {}", m)
//!     };
//! 
//!     // Get the `my_tree` model.
//!     if let Some(tree_model) = forest.tree(&String::from("my_tree")) {
//!         // Traverse the tree using the BFS algorithm.
//!         for n in tree_model.bfs_iter() {
//!             // Print the node content
//!             println!("{}", n.content.get_content());
//!         }
//!     }
//! }
//! ```
//! 
//! Generate a forest programatically and serialize it into a TREF file:
//! 
//! ```
//! let mut forest: Forest<SimpleNode> = Forest::empty();
//! // Create new tree and root node
//! let tree_id = String::from("my_tree");
//! forest.new_tree(&tree_id);
//! let _root = forest.set_root(&tree_id, &String::from("root_node")).unwrap();
//! // Add 3 children to root
//! let _node_1 = forest.link_node(&tree_id, _root, &String::from("node_1")).unwrap();
//! let _node_2 = forest.link_node(&tree_id, _root, &String::from("node_2")).unwrap();
//! let _node_3 = forest.link_node(&tree_id, _root, &String::from("node_3")).unwrap();
//! // Add 1 child to node_3
//! let _node_3_1 = forest.link_node(&tree_id, _node_3, &String::from("node_3_1")).unwrap();
//! // Add 2 children to node_1
//! let _node_1_1 = forest.link_node(&tree_id, _node_1, &String::from("node_1_1")).unwrap();
//! let _node_1_2 = forest.link_node(&tree_id, _node_1, &String::from("node_1_2")).unwrap();
//! // Serialize
//! let f = File::create("serialized.tref").expect("Unable to create file");
//! let mut buf_writer = BufWriter::new(f);
//! 
//! if !forest.serialize(&mut buf_writer) {
//!     println!("Failed serializing tree!");
//! }
//! ```
//! 
//! Is also possible to modify a tree parsed from a file:
//! 
//! ```
//! let file = File::open("simpletree.tref").unwrap();
//! let mut forest: Forest<SimpleNode> = Forest::build(BufReader::new(file)).unwrap();
//! // Add `child_4` to root.
//! let _child_4 = forest.link_node(&String::from("my_tree"), 0, &String::from("child_4")).unwrap();
//! ```
//! 
//! # Dialects
//! 
//! TREF also supports user defined dialects, that are trees that have nodes with a specific format. This is achived using the [`NodeContent`] trait.
//! 
//! For example, imagine we want to model a tree that has nodes of type string and others of type integer. Something like:
//! 
//! ```
//! [my_dialect_tree]
//! + root
//! + + string child
//! + + + 2500
//! + + + 130
//! ```
//! 
//! First we have to define a [`NodeContent`] to parse our custom nodes:
//! 
//! ```
//! enum TypedNode {
//!     Text(String),
//!     Number(String, u32)
//! }
//! 
//! impl NodeContent for TypedNode {
//!     fn new(content: String) -> Option<Self> {
//!         // Try to parse the node content as integer, if it fails, then it must be a string
//!         match content.trim().parse() {
//!             Ok(num) => Some(Self::Number(content, num)),
//!             Err(_) => Some(Self::Text(content))
//!         }
//!     }
//! 
//!     fn get_content(&self) -> &String {
//!         match self {
//!             Self::Text(t) => t,
//!             Self::Number(t, _) => t
//!         }
//!     }
//! }
//! ```
//! 
//! And then use it to parse the tree:
//! 
//! ```
//! let forest: Result<Forest<TypedNode>, String> = Forest::build(buf_reader);
//! ```
//! 
//! The [`NodeContent::new()`] is called every time a node of the tree is parsed. It returns an [`Option`], that means it can be None, in which case the TREF parser will fail, returing an error.

mod parser;
mod stack;
mod tree;
mod iter;
mod forest;

pub use tree::{NodeContent, SimpleNode, TreeModel, TreeNode};
pub use forest::*;

/*
TODO:
- All iterators must return a tuple with node and node index.
- Methods that return a Result should generate an error type implementing the std::error::Error trait, instead of a simple String.
- Allow using the BufReader directly to read data from the tree, instead of parsing and generating a model in mem. For very big trees.
*/

#[cfg(test)]
mod tests;