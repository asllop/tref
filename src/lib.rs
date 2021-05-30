mod parser;
mod stack;
mod tree;
mod iter;
mod forest;

pub use tree::{NodeContent, SimpleNode};
pub use forest::*;

/*
TODO:
- Delete nodes -> Deleting nodes is a bit more tricky, to avoid recalculating all indexes we must leave the index of the removed node empty, putting an "empty" value there. Using Option<u32> is the most elegant way, but costly, because we have to unwrap every time we access a node index. Maybe using u32::MAX value as empty is an acceptable choice. How will this affect iter()?
- Allow using the BufReader directly to read data from the tree, instead of parsing and generating a model in mem. For very big trees.
*/

#[cfg(test)]
mod tests;