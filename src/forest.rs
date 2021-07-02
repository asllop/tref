use std::io::{prelude::*, BufReader, BufWriter};
use std::collections::HashMap;
use crate::tree;
use crate::parser;
use crate::stack;

/// Contains the interfaces to parse, generate, modify and serialize TREF models.
#[derive(Debug)]
pub struct Forest<T: tree::NodeContent> {
    /// Hash map with all the trees contained in the Forest.
    pub trees: HashMap<String, tree::Tree<T>>,
    /// Optional hash map that contains information about tree levels, to accelerate and simplify some iterators.
    pub levels: Option<HashMap<String, Vec<tree::TreeLevel>>>
}

/// Generic `T` is a struct conforming to [`NodeContent`](`tree::NodeContent`) trait.
impl<T: tree::NodeContent> Forest<T> {

    /// Parse a TREF file and generate a Forest (without [`levels`][`Forest::levels`]).
    /// 
    /// # Arguments
    /// 
    /// * `reader` - Buffer reader where to obtsain the TREF file.
    ///
    /// # Return
    /// 
    /// * A [`Result`] with either the Forest representing the parsed TREF or a String describing an error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let buf_reader = ... // generate a std::io::BufReader
    /// let forest: Result<Forest<SimpleNode>, String> = Forest::build(buf_reader);
    /// match forest {
    ///     Ok(forest) => {
    ///         // Do whatever with the forest...
    ///         // ...
    ///     },
    ///     Err(msg) => {
    ///         println!("Error parsing TREF: {}", msg);
    ///         // ...
    ///     }
    /// }
    /// ```
    /// 
    pub fn build(reader: BufReader<impl Read>) -> Result<Self, String> {
        return Self::new(reader, false);
    }

    /// Parse a TREF file and generates a Forest with [`levels`][`Forest::levels`].
    /// 
    /// # Arguments
    /// 
    /// * `reader` - Buffer reader where to obtsain the TREF file.
    ///
    /// # Return
    /// 
    /// * A [`Result`] with either the Forest representing the parsed TREF or a String describing an error.
    /// 
    /// # Examples
    /// 
    /// build_levels is used like [`Forest::build()`].
    /// 
    pub fn build_levels(reader: BufReader<impl Read>) -> Result<Self, String> {
        return Self::new(reader, true);
    }

    /// Returns a tree model.
    /// 
    /// # Arguments
    /// 
    /// * `tree_id` - ID of the tree.
    ///
    /// # Return
    /// 
    /// * An [`Option`] with the tree model.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let tree_id = &String::from("my_tree");
    /// if let Some(tree_model) = forest.tree(tree_id) {
    ///     // ...
    /// }
    /// else {
    ///     println!("Tree with ID {} not found", tree_id);
    /// }
    /// ```
    /// 
    pub fn tree(&self, tree_id: &String) -> Option<tree::TreeModel<T>> {
        tree::TreeModel::new(self, tree_id)
    }

    /// Genereta an empty forest.
    ///
    /// # Return
    /// 
    /// * An empty Forest.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut forest: Forest<SimpleNode> = Forest::empty();
    /// ```
    /// 
    pub fn empty() -> Self {
        Forest { trees: HashMap::new(), levels: None }
    }

    /// Add a new empty tree to the forest.
    ///
    /// # Arguments
    /// 
    /// * `tree_id` - ID of the tree.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut forest: Forest<SimpleNode> = Forest::empty();
    /// 
    /// // Create new empty tree
    /// let tree_id = &String::from("my_tree");
    /// forest.new_tree(tree_id)
    /// ```
    /// 
    pub fn new_tree(&mut self, tree_id: &String) {
        self.add_tree(tree_id, tree::Tree::new());
    }

    /// Set the root node of a tree.
    /// 
    /// # Arguments
    /// 
    /// * `tree_id` - ID of the tree.
    /// * `root_node_content` - Content of the node.
    ///
    /// # Return
    /// 
    /// * A [`Result`] with either the index of the node created or a String describing an error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut forest: Forest<SimpleNode> = Forest::empty();
    /// let tree_id = String::from("my_tree");
    /// forest.new_tree(&tree_id);
    /// 
    /// // Set root node to tree
    /// let _root = forest.set_root(&tree_id, &String::from("root_node")).unwrap();
    /// ```
    /// 
    pub fn set_root(&mut self, tree_id: &String, root_node_content: &String) -> Result<u32, String> {
        if let Some(tree) = self.get_mut_tree(&tree_id) {
            if tree.add_root_node(&root_node_content) {
                Ok(0)
            }
            else {
                Result::Err(String::from("Failed parsing root node"))
            }
        }
        else {
            Result::Err(String::from("Tree ID not found"))
        }
    }

    /// Create a child node and link it to its parent.
    /// 
    /// # Arguments
    /// 
    /// * `tree_id` - ID of the tree.
    /// * `node_index` - Parent node index.
    /// * `node_content` - Child node content.
    ///
    /// # Return
    /// 
    /// * A [`Result`] with either the index of the node created or a String describing an error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut forest: Forest<SimpleNode> = Forest::empty();
    /// let tree_id = String::from("my_tree");
    /// forest.new_tree(&tree_id);
    /// let _root = forest.set_root(&tree_id, &String::from("root_node")).unwrap();
    /// 
    /// // Add two children to root node
    /// let _node_1 = forest.link_node(&tree_id, _root, &String::from("node_1")).unwrap();
    /// let _node_2 = forest.link_node(&tree_id, _root, &String::from("node_2")).unwrap();
    /// ```
    /// 
    pub fn link_node(&mut self, tree_id: &String, node_index: u32, node_content: &String) -> Result<u32, String> {
        if let Some(tree) = self.get_mut_tree(&tree_id) {
            if tree.nodes.len() > node_index as usize {
                let parent_level = tree.nodes[node_index as usize].level;
                let parent_node_ref = stack::NodeStackContent::new(parent_level, node_index);
                let new_node = tree.add_node(&node_content, parent_level + 1, &parent_node_ref, Some(tree.nodes[node_index as usize].children.len() as u32));
                //Add child node to parent
                tree.nodes[node_index as usize].add_child_node(new_node);
                Ok(new_node)
            }
            else {
                Result::Err(String::from("Node index not found"))
            }
        }
        else {
            Result::Err(String::from("Tree ID not found"))
        }
    }

    /// Unlink a child node.
    /// 
    /// # Arguments
    /// 
    /// * `tree_id` - ID of the tree.
    /// * `node_index` - Indedx of the node to unlink.
    ///
    /// # Return
    /// 
    /// * A [`Result`] with either the index of the unlinked node or a String describing an error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut forest: Forest<SimpleNode> = Forest::empty();
    /// let tree_id = String::from("my_tree");
    /// forest.new_tree(&tree_id);
    /// let _root = forest.set_root(&tree_id, &String::from("root_node")).unwrap();
    /// let _node_1 = forest.link_node(&tree_id, _root, &String::from("node_1")).unwrap();
    /// let _node_2 = forest.link_node(&tree_id, _root, &String::from("node_2")).unwrap();
    /// 
    /// // Unlink node_1 from root node
    /// forest.unlink_node(&tree_id, _node_1).unwrap();
    /// ```
    /// 
    pub fn unlink_node(&mut self, tree_id: &String, node_index: u32) -> Result<u32, String> {
        if let Some(tree) = self.get_mut_tree(&tree_id) {
            if tree.nodes.len() > node_index as usize {
                if let Some(parent) = tree.nodes[node_index as usize].parent_position {
                    if let Some(parent_children_pos) = tree.nodes[node_index as usize].parent_children_pos {
                        if tree.nodes[parent as usize].children.len() > parent_children_pos as usize {
                            tree.nodes[parent as usize].children.remove(parent_children_pos as usize);
                            Ok(node_index)
                        }
                        else {
                            Result::Err(String::from("Node not found in parent node children array"))
                        }
                    }
                    else {
                        Result::Err(String::from("parent_children_pos is none"))
                    }
                }
                else {
                    Result::Err(String::from("Trying to unlink the root node"))
                }
            }
            else {
                Result::Err(String::from("Node index not found"))
            }
        }
        else {
            Result::Err(String::from("Tree ID not found"))
        }
    }

    fn find_child(nodes: &Vec<tree::TreeNode<T>>, parent: u32, child_content: &String) -> Option<u32> {
        if nodes.len() > parent as usize {
            for n in &nodes[parent as usize].children {
                if nodes.len() > *n as usize {
                    if nodes[*n as usize].content.get_content() == child_content {
                        return Some(*n);
                    }
                }
            }
        }
        None
    }

    /// Find a node specifing the path from the root.
    /// 
    /// # Arguments
    /// 
    /// * `tree_id` - ID of the tree.
    /// * `path` - Vector containing the nodes of the path.
    ///
    /// # Return
    /// 
    /// * An [`Option`] with the node index.
    /// 
    /// # Examples
    /// 
    /// ```
    /// // Path of node: root_node -> child_2 -> child_2_1 -> child_2_1_1
    /// let child_2_1_1 = forest.find_node(&String::from("my_tree"), vec!(String::from("root_node"), String::from("child_2"), String::from("child_2_1"), String::from("child_2_1_1"))).unwrap();
    /// ```
    /// 
    pub fn find_node(&self, tree_id: &String, path: Vec<String>) -> Option<u32> {
        let mut current_node: u32 = 0;
        let mut current_path_pos: u32 = 0;
        if let Some(tree) = self.tree(&tree_id) {
            // Check if root node matches
            if path.len() > current_path_pos as usize {
                if tree.tree_ref.nodes.len() > current_node as usize{
                    if tree.tree_ref.nodes[current_node as usize].content.get_content() != &path[current_path_pos as usize] {
                        return None;
                    }
                }
                else {
                    return None;
                }
            }
            else {
                return None;
            }
            // check the rest of nodes
            loop {
                current_path_pos = current_path_pos + 1;
                if path.len() > current_path_pos as usize {
                    if let Some(n) = Self::find_child(&tree.tree_ref.nodes, current_node, &path[current_path_pos as usize]) {
                        current_node = n;
                    }
                    else {
                        // Child not found
                        return None;
                    }
                }
                else {
                    // End of path, return what we have
                    return Some(current_node);
                }
            }
        }
        else {
            return None;
        }
    }

    /// Convert a Forest structure into a TREF file.
    /// 
    /// # Arguments
    /// 
    /// * `buf_writer` - BufWriter where to write the TREF.
    ///
    /// # Return
    /// 
    /// * A boolean, true if serialization was ok, false if not.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let f = File::create("./serialized.tref").expect("Unable to create file");
    /// let mut buf_writer = BufWriter::new(f);
    /// 
    /// if !forest.serialize(buf_writer) {
    ///     println!("Failed serializing TREF");
    /// }
    /// ```
    /// 
    pub fn serialize(&self, buf_writer: &mut BufWriter<impl Write>) -> bool {
        let parser = parser::TreeParser::new();
        for (tree_id, _) in self.trees.iter() {
            // write tree id statement
            let tree_id_statement = format!("[{}]", tree_id);
            if let parser::TreeStatement::TreeID(_) = parser.parse_statement(&tree_id_statement) {
                if let Err(_) = buf_writer.write(&format!("{}\n",tree_id_statement).as_bytes()) {
                    return false;
                }
            }
            else {
                return false;
            }
            // iter all nodes and generate statements
            let tree_model = self.tree(tree_id).unwrap();
            for n in tree_model.pre_dfs_iter() {
                let mut node_statement = String::new();
                for _ in 0..n.level {
                    node_statement.push_str("+ ");
                }
                node_statement.push_str(&format!("{}", n.content.get_content()));
                // write node
                if let parser::TreeStatement::Node(_,_) = parser.parse_statement(&node_statement) {
                    if let Err(_) = buf_writer.write(format!("{}\n", node_statement).as_bytes()) {
                        return false;
                    }
                }
                else {
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
                                    let parent_children_pos;
                                    if let Some(parent_node) = tree.get_mut_node(&parent_node_ref) {
                                        parent_children_pos = Some(parent_node.children.len() as u32);
                                    }
                                    else {
                                        return Result::Err(format!("Couldn't find a parent node at line {}", i + 1));
                                    }
                                    // Put new node in the tree
                                    let new_node_position = tree.add_node(&content, level, &parent_node_ref, parent_children_pos);
                                    if new_node_position == 0 {
                                        return Result::Err(format!("Failed parsing node at line {}", i + 1));
                                    }
                                    // Update levels
                                    if use_levels {
                                        Self::add_node_to_levels(&mut levels, &current_tree_id, level, new_node_position)?;
                                    }
                                    // Attach node to parent
                                    tree.get_mut_node(&parent_node_ref).unwrap().add_child_node(new_node_position);
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