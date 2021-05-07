use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

mod parser;
mod stack;
mod tree;
mod iter;

#[derive(Debug)]
pub struct Forest {
    trees: HashMap<String, tree::Tree>,
    levels: HashMap<String, Vec<tree::TreeLevel>>
}

impl Forest {
    //TODO: return an error type implementing the std::error::Error trait.
    pub fn new(reader: BufReader<impl Read>) -> Result<Self, String> {
        let parser = parser::TreeParser::new();
        let mut stack = stack::NodeStack::new();
        let mut prev_level:u32 = 0;
        let mut current_tree_id = String::new();
        let mut forest = Forest { trees: HashMap::new(), levels: HashMap::new() };
        let mut levels: HashMap<String, Vec<tree::TreeLevel>> = HashMap::new();
    
        for (i, l) in reader.lines().enumerate() {
            if let Ok(line) = l {
                let statement = parser.parse_statement(&line);
                match statement {
                    parser::TreeStatement::Invalid => return Result::Err(format!("Invalid statement at line {}", i + 1)),
                    parser::TreeStatement::TreeID(tree_id) => {
                        stack.flush();
                        current_tree_id = tree_id;
                    }
                    parser::TreeStatement::Node(content, level) => {
                        if level > prev_level + 1 {
                            return Result::Err(format!("Invalid node level at line {}", i + 1));
                        }
    
                        // Root node
                        if level == 1 {
                            if let Some(_) = stack.top() {
                                return Result::Err(format!("Multiple root nodes for the same tree at line {}", i + 1));
                            }

                            if current_tree_id.is_empty() {
                                return Result::Err(format!("Found root node without previous tree ID at line {}", i + 1));
                            }

                            // Create new tree with root node and add to forest
                            let tree = tree::Tree::new(&content);
                            forest.add_tree(&current_tree_id, tree);
                            // Update levels
                            Self::add_node_to_levels(&mut levels, &current_tree_id, level, 0)?;
    
                            // Push root node reference to stack
                            stack.push_new(1, 0);
                        }
                        // Somebody's child node
                        else {
                            if let Some(parent_node_ref) = stack.pop_parent(level) {
                                if let Some(tree) = forest.get_mut_tree(&current_tree_id) {
                                    // Put new node in the tree
                                    let new_node_position = tree.add_node(&content, level, &parent_node_ref);
                                    // Update levels
                                    Self::add_node_to_levels(&mut levels, &current_tree_id, level, new_node_position)?;

                                    // Attach node to parent
                                    if let Some(parent_node) = tree.get_mut_node(&parent_node_ref) {
                                        parent_node.add_child_node(new_node_position);
                                    }
                                    else {
                                        return Result::Err(format!("Couldn't find a parent node at line {}", i + 1));
                                    }

                                    // Push back parent node reference to stack
                                    stack.push(parent_node_ref);
                                    // Push new node reference to stack
                                    stack.push_new(level, new_node_position);
                                }
                                else {
                                    return Result::Err(format!("Couldn't find tree at line {}", i + 1));
                                }
                            }
                            else {
                                return Result::Err(format!("Couldn't find a parent ref at line {}", i + 1));
                            }
                        }
    
                        prev_level = level;
                    },
                    _ => {}
                }
            }
            else {
               return Result::Err(format!("Could not read line at {}", i + 1));
            }
        }
    
        forest.levels = levels;
        Result::Ok(forest)
    }  
    
    fn add_node_to_levels(levels: &mut HashMap<String, Vec<tree::TreeLevel>>, tree_id: &String, level: u32, node_pos: u32) -> Result<(), String> {
        // Tree doesn't exist, create it
        if let None = levels.get_mut(tree_id) {
            levels.insert(String::from(tree_id), vec!());
        }
        // Get tree vector
        if let Some(tree_level_vec) = levels.get_mut(tree_id) {
            // Level doesn't exist, create it
            if let None = tree_level_vec.get_mut(level as usize - 1) {
                tree_level_vec.push(tree::TreeLevel {
                    level,
                    node_positions: vec!()
                });
            }
            // Add node_position to level
            if let Some(tree_level) = tree_level_vec.get_mut(level as usize - 1) {
                tree_level.node_positions.push(node_pos);
            }
            else {
                return Result::Err(format!("Level tree vector position not found"));
            }
        }
        else {
            return Result::Err(format!("Level tree vector not found"));
        }

        Ok(())
    }

    fn add_tree(&mut self, tree_id: &String, tree: tree::Tree) {
        self.trees.insert(String::from(tree_id), tree);
    }

    fn get_mut_tree(&mut self, current_tree_id: &String) -> Option<&mut tree::Tree> {
        self.trees.get_mut(current_tree_id)
    }

    pub fn tree(&self, tree_id: &String) -> Option<tree::TreeModel> {
        tree::TreeModel::new(self, tree_id)
    }
}

/*
TODO:
- Generate tree with default object or user defined. La callback de l'usr rep: node str, parent node i depth.
- Create iterators to traverse the tree using various algorithms (https://towardsdatascience.com/4-types-of-tree-traversal-algorithms-d56328450846 , https://en.wikipedia.org/wiki/Tree_traversal).
- Find a specific node, starting on any node or root.
- Access a specific node by using a path.
- Allow using the BufReader directly to read data from the tree, instead of parsing and generating a model in mem. For very big trees.
- Generate a tree/forest programatically and serialize into a TREF file.
*/

#[cfg(test)]
mod tests;