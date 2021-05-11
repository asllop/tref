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
- Modify trees -> adding or moving nodes is trivial. Deleting nodes is a bit more tricky, to avoid recalculating all indexes we must leave the index of the removed node empty, putting an "empty" value there. Using Option<u32> is the most elegant way, but costly, because we have to unwrap every time we access a node index. Maybe using u32::MAX value as empty is an acceptable choice. How will this affect iter()?
- Allow using the BufReader directly to read data from the tree, instead of parsing and generating a model in mem. For very big trees.
*/

#[cfg(test)]
mod tests;