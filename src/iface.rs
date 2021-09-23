use std::io::{prelude::*, BufReader};
use std::marker::PhantomData;
use socarel::{Forest, NodeContent, RawNode};
use crate::stack::*;
use crate::parser::*;

/// Document interaction model.
pub struct Model<T: NodeContent = RawNode> {
    phantom: PhantomData<T>
}

impl<T: NodeContent> Model<T> {
    /// Create new parser.
    pub fn new() -> Self {
        Model { phantom: PhantomData }
    }

    /// Parse TREF document.
    /// 
    /// # Arguments
    /// 
    /// * `reader` - BufReader to read the document.
    /// 
    /// # Return
    /// 
    /// * A [`Result`] with a `Forest` or a `ParseTreeError`.
    ///
    pub fn parse(&self, reader: BufReader<impl Read>) -> Result<Forest<T>, ParseTreeError> {
        let parser = TreeParser::new();
        let mut stack = NodeStack::new();
        let mut prev_level = 0;
        let mut current_tree_id = String::new();
        let mut forest = Forest::<T>::new();

        for (i, l) in reader.lines().enumerate() {
            if let Ok(line) = l {
                let statement = parser.parse_statement(&line);
                match statement {
                    TreeStatement::Invalid => {
                        return Result::Err(ParseTreeError::new("Invalid statement", i))
                    },
                    TreeStatement::TreeID(tree_id) => {
                        stack.flush();
                        // Create new tree
                        forest.new_tree(&tree_id);
                        current_tree_id = tree_id;
                    }
                    TreeStatement::Node(content, level) => {
                        if level > prev_level + 1 {
                            return Result::Err(ParseTreeError::new("Invalid node level", i));
                        }
    
                        // Root node
                        if level == 1 {
                            if let Some(_) = stack.top() {
                                return Result::Err(ParseTreeError::new("Multiple root nodes for the same tree", i));
                            }

                            if let Some(tree) = forest.get_mut_tree(&current_tree_id) {
                                // Create root node
                                if let None = tree.set_root(&content) {
                                    return Result::Err(ParseTreeError::new("Failed parsing root node", i));
                                }
                                // Push root node reference to stack
                                stack.push_new(1, 0);
                            }
                            else {
                                return Result::Err(ParseTreeError::new("Found root node without previous tree ID", i));
                            }
                        }
                        // Somebody's child node
                        else {
                            if let Some(parent_node_ref) = stack.pop_parent(level) {
                                if let Some(tree) = forest.get_mut_tree(&current_tree_id) {
                                    if let Some(new_node) = tree.link_node(&content, parent_node_ref.tree_position) {
                                        // Push back parent node reference to stack
                                        stack.push(parent_node_ref);
                                        // Push new node reference to stack
                                        stack.push_new(level, new_node);
                                    }
                                    else {
                                        return Result::Err(ParseTreeError::new("Failed parsing node", i));
                                    }
                                }
                                else {
                                    return Result::Err(ParseTreeError::new("Couldn't find tree", i));
                                }
                            }
                            else {
                                return Result::Err(ParseTreeError::new("Couldn't find a parent ref", i));
                            }
                        }
    
                        prev_level = level;
                    },
                    _ => {}
                }
            }
            else {
                return Result::Err(ParseTreeError::new("Could not read line", i));
            }
        }

        Result::Ok(forest)
    }

    //TODO: implement serializer
}