mod parser;
mod stack;
mod tree;
mod iter;
mod forest;

pub use tree::{NodeContent, SimpleNode};
pub use forest::*;

/*
TODO:
- Find a specific node, starting at any node or root.
- Access a specific node by using a path.
- Generate a tree/forest programatically and serialize into a TREF file.
- Allow using the BufReader directly to read data from the tree, instead of parsing and generating a model in mem. For very big trees.
*/

#[cfg(test)]
mod tests;