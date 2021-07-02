//! # TREF
//!
//! TREF is a plain text file format to describe trees in a human readable way.
//! 
//! With TREF a human can write a tree and understand the structure by having a quick look, because it is designed to be both easy to read for humans and easy to parse for machines.
//! 
//! Writing a tree in a file can be useful for many reasons: as a config file for an application, to store information that can be modified and read by an app and its user, to serialize tree-like memory structures, etc.
//! 
//! # Examples
//! 
//! TODO: examples

mod parser;
mod stack;
mod tree;
mod iter;
mod forest;

pub use tree::{NodeContent, SimpleNode, TreeModel, TreeNode};
pub use forest::*;

/*
TODO:
- Methods that return a Result should generate an error type implementing the std::error::Error trait, instead of a simple String.
- Allow using the BufReader directly to read data from the tree, instead of parsing and generating a model in mem. For very big trees.
*/

#[cfg(test)]
mod tests;