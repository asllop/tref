use std::io::{prelude::*, BufReader, BufWriter};
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
                                return Result::Err(ParseTreeError::new("Multiple root nodes in the same tree", i));
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

    /// Convert a Forest structure into a TREF document.
    /// 
    /// # Arguments
    /// 
    /// * `forest` - Reference to a `Forest`.
    /// * `writer` - BufWriter where to write the TREF.
    ///
    /// # Return
    /// 
    /// * A boolean, true if serialization was ok, false if not.
    /// 
    pub fn serialize(&self, forest: &Forest<T>, writer: &mut BufWriter<impl Write>) -> bool {
        let parser = TreeParser::new();
        for (tree_id, _) in forest.iter() {
            // write tree id statement
            let tree_id_statement = format!("[{}]", tree_id);
            if let TreeStatement::TreeID(_) = parser.parse_statement(&tree_id_statement) {
                if let Err(_) = writer.write(&format!("{}\n",tree_id_statement).as_bytes()) {
                    return false;
                }
            }
            else {
                return false;
            }
            // iter all nodes and generate statements
            if let Some(tree) = forest.get_tree(tree_id) {
                for (n, _) in tree.iterators().pre_dfs() {
                    let mut node_statement = String::new();
                    for _ in 0..n.get_level() {
                        node_statement.push_str("+ ");
                    }
                    node_statement.push_str(&format!("{}", n.get_content_ref().gen_content()));
                    // write node
                    if let TreeStatement::Node(_,_) = parser.parse_statement(&node_statement) {
                        if let Err(_) = writer.write(format!("{}\n", node_statement).as_bytes()) {
                            return false;
                        }
                    }
                    else {
                        return false;
                    }
                }
            }
            else {
                return false;
            }
        }
        if let Err(_) = writer.flush() {
            false
        }
        else {
            true
        }
    }
}