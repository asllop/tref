use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

mod parser {
    use regex::Regex;

    pub enum TreeStatement {
        TreeID(String),
        Node(String, u32),
        Comment,
        Empty,
        Invalid
    }
    
    pub struct TreeParser {
        tree_id_matcher: Regex,
        tree_id_finder: Regex,
        node_matcher: Regex,
        node_finder: Regex,
        node_level_finder: Regex,
        comment_matcher: Regex
    }
    
    impl TreeParser {
        pub fn new() -> Self {
            Self {
                tree_id_matcher: Regex::new(r"^\[[A-Za-z0-9_]+\]$").unwrap(),
                tree_id_finder: Regex::new(r"[A-Za-z0-9_]+").unwrap(),
                node_matcher: Regex::new(r"^(\+ )+[^\+ ].*$").unwrap(),
                node_finder: Regex::new(r"(\+ )+").unwrap(),
                node_level_finder: Regex::new(r"(\+ )").unwrap(),
                comment_matcher: Regex::new(r"^#.*+$").unwrap()
            }
        }
    
        pub fn parse_statement(&self, statement: &String) -> TreeStatement {
            if self.node_matcher.is_match(statement) {
                let n = self.node_finder.find(statement).unwrap();
                let node = &statement[n.end()..];
                let level_iter = self.node_level_finder.find_iter(statement);
                let mut level = 0;
                for _ in level_iter { level += 1 }
                TreeStatement::Node(String::from(node), level)
            }
            else if self.tree_id_matcher.is_match(statement) {
                let n = self.tree_id_finder.find(statement).unwrap();
                let tree_id = &statement[n.start()..n.end()];
                TreeStatement::TreeID(String::from(tree_id))
            }
            else if self.comment_matcher.is_match(statement) {
                TreeStatement::Comment
            }
            else {
                if statement.len() == 0 {
                    TreeStatement::Empty
                }
                else if statement.trim().len() == 0 {
                    TreeStatement::Empty
                }
                else {
                    TreeStatement::Invalid
                }
            }
        }
    }
}

mod stack {
    #[derive(Debug)]
    pub struct NodeStack {
        buffer: Vec<NodeStackContent>
    }

    #[derive(Debug)]
    pub struct NodeStackContent {
        level: u32,
        pub tree_position: u32
    }

    impl NodeStack {
        pub fn new() -> Self {
            Self {
                buffer: Vec::new()
            }
        }

        pub fn push(&mut self, obj: NodeStackContent) {
            self.buffer.push(obj);
        }

        pub fn pop(&mut self) -> Option<NodeStackContent> {
            self.buffer.pop()
        }

        pub fn pop_parent(&mut self, level: u32) -> Option<NodeStackContent> {
            // Obtain data from stack until we get one node with a level lower than "level"
            loop {
                let n = self.pop();
                if let Some(n_node) = n {
                    if n_node.level < level {
                        return Some(n_node);
                    }
                }
                else {
                    return None;
                }
            }
        }

        pub fn flush(&mut self) {
            self.buffer.truncate(0);
        }

        pub fn top(&mut self) -> Option<&NodeStackContent> {
            self.buffer.last()
        }
    }

    impl NodeStackContent {
        pub fn new(level: u32, tree_position: u32) -> Self {
            Self {
                level,
                tree_position
            }
        }
    }
}

/*
TODO:
Create a data structure for level by level access. This struct makes reference to the Tree struct.
Will contain one position of an array per level, and each position will contain references to all nodes of that level.
Only for Breadth-first trasverse
*/
mod tree {
    #[derive(Debug)]
    pub struct TreeNode {
        pub content: String,
        pub level: u32,
        pub parent_position: Option<u32>,
        pub children: Vec<u32>
    }

    #[derive(Debug)]
    pub struct Tree {
        pub nodes: Vec<TreeNode>
    }

    #[derive(Debug)]
    pub struct TreeLevel {
        pub level: u32,
        pub node_positions: Vec<u32>
    }

    impl TreeNode {
        pub fn new(content: &String, level: u32, parent_position: Option<u32>) -> Self {
            Self {
                content: String::from(content),
                level,
                parent_position,
                children: vec!()
            }
        }

        pub fn new_root(content: &String) -> Self {
            Self::new(content, 1, None)
        }
    }

    #[derive(Debug)]
    pub struct TreeModel<'a> {
        tree_ref:  &'a Tree,
        level_ref: &'a Vec<TreeLevel>
    }

    impl<'a> TreeModel<'a> {
        pub fn new(forest: &'a crate::Forest, tree_id: &String) -> Option<Self> {
            if let None = forest.trees.get(tree_id) {
                return None;
            }
            else if let None = forest.levels.get(tree_id) {
                return None;
            }
            else {
                if let Some(tree_ref) = forest.trees.get(tree_id) {
                    if let Some(level_ref) = forest.levels.get(tree_id) {
                        return Some(Self {
                            tree_ref,
                            level_ref
                        });
                    }
                }
            }
            None
        }
    }
}

mod iter {
    struct BfsIter<'a> {
        tree: crate::tree::TreeModel<'a>
    }

    impl<'a> Iterator for BfsIter<'a> {
        type Item = ();//&'a crate::tree::TreeNode;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }
}

#[derive(Debug)]
pub struct Forest {
    trees: HashMap<String, tree::Tree>,
    levels: HashMap<String, Vec<tree::TreeLevel>>
}

impl Forest {
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
                    parser::TreeStatement::Invalid => return Result::Err(format!("Invalid statement at line {}", i)),
                    parser::TreeStatement::TreeID(tree_id) => {
                        println!("-------------------------");
                        println!("tree_id       {}", tree_id);
                        stack.flush();
                        current_tree_id = tree_id;
                    }
                    parser::TreeStatement::Node(content, level) => {
                        println!("node          {} (level: {})", content, level);
    
                        if level > prev_level + 1 {
                            return Result::Err(format!("Invalid node level at line {}", i));
                        }
    
                        if level == 1 {
                            // Root node
                            println!("Root node");
                            
                            if let Some(_) = stack.top() {
                                return Result::Err(format!("Multiple root nodes at line {}", i));
                            }

                            // Create a new tree and put root node
                            let mut tree = tree::Tree { nodes: vec!() };
                            tree.nodes.push(tree::TreeNode::new_root(&content));
                            Self::add_node_to_levels(&mut levels, &current_tree_id, level, 0)?;
                            forest.trees.insert(String::from(&current_tree_id), tree);
    
                            // Put node reference on stack
                            stack.push(stack::NodeStackContent::new(level, 0));
                        }
                        else {
                            // Somebody's child node
                            if let Some(parent_node_ref) = stack.pop_parent(level) {
                                if let Some(tree) = forest.trees.get_mut(&current_tree_id) {
                                    // Put new node in the tree
                                    tree.nodes.push(tree::TreeNode::new(&content, level, Some(parent_node_ref.tree_position)));
                                    let new_node_position = tree.nodes.len() as u32 - 1;
                                    Self::add_node_to_levels(&mut levels, &current_tree_id, level, new_node_position)?;

                                    // Attach node to parent
                                    if let Some(parent_node) = tree.nodes.get_mut(parent_node_ref.tree_position as usize) {
                                        parent_node.children.push(new_node_position);
                                        println!("My parent is {}", parent_node.content);
                                    }
                                    else {
                                        return Result::Err(format!("Couldn't find a parent node at line {}", i));
                                    }

                                    // Push back parent node reference to stack
                                    stack.push(parent_node_ref);
                                    // Push new node reference to stack
                                    stack.push(stack::NodeStackContent::new(level, new_node_position));
                                }
                                else {
                                    return Result::Err(format!("Couldn't find tree at line {}", i));
                                }
                            }
                            else {
                                return Result::Err(format!("Couldn't find a parent ref at line {}", i));
                            }
                        }
    
                        prev_level = level;
                    },
                    _ => {}
                }
            }
            else {
               return Result::Err(format!("Could not read line at {}", i));
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

    pub fn tree(&mut self, tree_id: &String) -> Option<tree::TreeModel> {
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