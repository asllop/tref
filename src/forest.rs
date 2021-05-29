use std::io::{prelude::*, BufReader, BufWriter};
use std::collections::HashMap;
use crate::tree;
use crate::parser;
use crate::stack;

#[derive(Debug)]
pub struct Forest<T: tree::NodeContent> {
    pub trees: HashMap<String, tree::Tree<T>>,
    pub levels: Option<HashMap<String, Vec<tree::TreeLevel>>>
}

impl<T: tree::NodeContent> Forest<T> {
    //TODO: return an error type implementing the std::error::Error trait.

    pub fn build(reader: BufReader<impl Read>) -> Result<Self, String> {
        return Self::new(reader, false);
    }

    pub fn build_levels(reader: BufReader<impl Read>) -> Result<Self, String> {
        return Self::new(reader, true);
    }

    pub fn tree(&self, tree_id: &String) -> Option<tree::TreeModel<T>> {
        tree::TreeModel::new(self, tree_id)
    }

    pub fn empty() -> Self {
        Forest { trees: HashMap::new(), levels: None }
    }

    pub fn new_tree(&mut self, tree_id: &String, root_node_content: &String) -> Result<(), String> {
        let mut tree = tree::Tree::new();
        if tree.add_root_node(&root_node_content) {
            self.add_tree(tree_id, tree);
            Ok(())
        }
        else {
            Result::Err(String::from("Failed parsing root node"))
        }
    }

    pub fn link_node(&mut self, tree_id: &String, node_index: u32, node_content: &String) -> Option<u32> {
        if let Some(tree) = self.get_mut_tree(&tree_id) {
            if tree.nodes.len() > node_index as usize {
                let parent_level = tree.nodes[node_index as usize].level;
                let parent_node_ref = stack::NodeStackContent::new(parent_level, node_index);
                let new_node = tree.add_node(&node_content, parent_level + 1, &parent_node_ref);
                //Add child node to parent
                tree.nodes[node_index as usize].children.push(new_node);
                Some(new_node)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    pub fn serialize(&self, mut buf_writer: BufWriter<impl Write>) -> bool {
        for (tree_id, _) in self.trees.iter() {
            // write tree id statement
            if let Err(_) = buf_writer.write(&format!("[{}]\n", tree_id).as_bytes()) {
                return false;
            }
            // iter all nodes and generate statements
            let tree_model = self.tree(tree_id).unwrap();
            for n in tree_model.pre_dfs_iter() {
                let mut node_statement = String::new();
                for _ in 0..n.level {
                    node_statement.push_str("+ ");
                }
                node_statement.push_str(&format!("{}\n", n.content.get_content()));
                // write node
                if let Err(_) = buf_writer.write(node_statement.as_bytes()) {
                    return false;
                }
            }
        }
        if let Err(_) = buf_writer.flush() {
            false
        }
        else {
            true
        }
    }

    fn new(reader: BufReader<impl Read>, use_levels: bool) -> Result<Self, String> {
        let parser = parser::TreeParser::new();
        let mut stack = stack::NodeStack::new();
        let mut prev_level:u32 = 0;
        let mut current_tree_id = String::new();
        let mut forest = Forest::empty();
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
                            let mut tree = tree::Tree::new();
                            if !tree.add_root_node(&content) {
                                return Result::Err(format!("Failed parsing root node at line {}", i + 1));
                            }
                            forest.add_tree(&current_tree_id, tree);
                            // Update levels
                            if use_levels {
                                Self::add_node_to_levels(&mut levels, &current_tree_id, level, 0)?;
                            }
    
                            // Push root node reference to stack
                            stack.push_new(1, 0);
                        }
                        // Somebody's child node
                        else {
                            if let Some(parent_node_ref) = stack.pop_parent(level) {
                                if let Some(tree) = forest.get_mut_tree(&current_tree_id) {
                                    // Put new node in the tree
                                    let new_node_position = tree.add_node(&content, level, &parent_node_ref);
                                    if new_node_position == 0 {
                                        return Result::Err(format!("Failed parsing node at line {}", i + 1));
                                    }
                                    // Update levels
                                    if use_levels {
                                        Self::add_node_to_levels(&mut levels, &current_tree_id, level, new_node_position)?;
                                    }

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
    
        if use_levels {
            forest.levels = Some(levels);
        }

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

    fn add_tree(&mut self, tree_id: &String, tree: tree::Tree<T>) {
        self.trees.insert(String::from(tree_id), tree);
    }

    fn get_mut_tree(&mut self, current_tree_id: &String) -> Option<&mut tree::Tree<T>> {
        self.trees.get_mut(current_tree_id)
    }
}