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
//! ```tref
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
//! use tref;
//! ```
//! 
//! Parse the `file.tref` file, traverse `my_tree`, modify tree and serialize:
//! 
//! ```
//! use std::{fs::File, io::{BufReader, BufWriter}};
//! use socarel::{NodeContent};
//! 
//! if let Ok(file) = File::open("file.tref") {
//!     let model = <tref::Model>::new();
//!     // Parse document
//!     match model.parse(BufReader::new(file)) {
//!         Ok(mut forest) => {
//!             // Get the `my_tree` model.
//!             if let Some(tree) = forest.get_mut_tree("my_tree") {
//!                 // Traverse the tree using the BFS algorithm
//!                 for (n, _) in tree.iterators().bfs() {
//!                     // Print the node content
//!                     println!("{}", n.get_content_ref().get_val());
//!                 }
//! 
//!                 // Unlink node at index 1
//!                 tree.unlink_node(1);
//!             }
//! 
//!             // Serialize the resulting forest back into a TREF file
//!             let f = File::create("serialized.tref").expect("Unable to create file");
//!             let mut buf_writer = BufWriter::new(f);
//!             match model.serialize(&forest, &mut buf_writer) {
//!                 Ok(num_lines) => {
//!                     println!("Tree serialized correctly, num lines = {}", num_lines);
//!                 },
//!                 Err(e) => {
//!                     println!("Failed serializing tree: {}", e);
//!                 }
//!             }
//!         },
//!         Err(e) => {
//!             println!("Could not parse TREF: {}", e)
//!         }
//!     }
//! }
//! ```
//! 
//! The example above uses [`unlink_node`][`socarel::Tree::unlink_node`] to disconnect a node from the tree. To know more about how to manupulate trees, please check out the [`socarel`] crate documentation.
//! 
//! # Dialects
//! 
//! TREF also supports user defined dialects, that are trees that have nodes with a specific format. This is achived using the [`NodeContent`][`socarel::NodeContent`] trait.
//! 
//! For example, imagine we want to model a tree with nodes that can only have integer values. Something like:
//! 
//! ```tref
//! [my_dialect_tree]
//! + 1000
//! + + 800
//! + + + 2500
//! + + + 130
//! ```
//! 
//! First we have to define a [`NodeContent`][`socarel::NodeContent`] to parse our custom nodes:
//! 
//! ```
//! use socarel::NodeContent;
//! 
//! pub struct IntegerNode {
//!     num: i32,
//!     content: String
//! }
//! 
//! impl IntegerNode {
//!     pub fn get_num(&self) -> i32 {
//!         self.num
//!     }
//! }
//! 
//! impl NodeContent for IntegerNode {
//!     fn new(content: &str) -> Option<Self> {
//!         // Try to parse the node content as integer
//!         match content.trim().parse() {
//!             Ok(num) => {
//!                 let content = String::from(content);
//!                 Some(IntegerNode { num, content })
//!             },
//!             Err(_) => None
//!         }
//!     }
//! 
//!     fn get_val(&self) -> &str {
//!         &self.content
//!     }
//! 
//!     fn gen_content(&self) -> String {
//!         String::from(self.get_val())
//!     }
//! }
//! 
//! // And then use it to parse the tree:
//! 
//! let model = tref::Model::<IntegerNode>::new();
//! // call model.parse(...), etc. Now all nodes inside the tree will be of type `IntegerNode`.
//! ```
//! 
//! The [`NodeContent::new()`][`socarel::NodeContent::new()`] is called every time a node of the tree is parsed. It returns an [`Option`], that means it can be None, in which case the TREF parser will fail, returing an error.

mod parser;
mod stack;
mod iface;
mod error;

pub use iface::*;
pub use error::*;

#[cfg(test)]
mod tests;