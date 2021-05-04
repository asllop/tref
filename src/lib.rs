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
                node_matcher: Regex::new(r"^(\+ )+[^\+].*$").unwrap(),
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
        //TODO: remove "node" field, not needed
        pub node: String,
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
        pub fn new(node: &String, level: u32, tree_position: u32) -> Self {
            Self {
                node: String::from(node),
                level,
                tree_position
            }
        }
    }
}

/*
#[derive(Debug)]
pub struct TreeNode<'a> {
    pub children: Vec<TreeNode<'a>>,
    pub level: u32,
    pub parent: Option<&'a TreeNode<'a>>,
    pub content: String
}
*/

//Data structure for level by level access. This struct makes reference to the TreeNode struct, that is the main structure to hold the nodes and the tree.
// only for Breadth-first trasverse
/*
#[derive(Debug)]
pub struct LevelTreeNode<'a> {
    pub nodes: Vec<Option<&'a TreeNode<'a>>>,
    pub level: u32
}
*/

/*
Alternative structure for Tree:

- An array of structs (NodeStruct)
- The pos 0 is always the root.
- The NodeStruct contains node name, and an array of node positions, which are array positions where the childs are located.
- This way we don't have references that can hanging references.
- In the stack, the NodeStackContent contains an array position to the NodeStruct and a level.
 */

#[derive(Debug)]
struct TreeNode {
    content: String,
    level: u32,
    parent_position: Option<u32>,
    children: Vec<u32>
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<TreeNode>
}

#[derive(Debug)]
pub struct Forest {
    trees: HashMap<String, Tree>
}

//TODO: create type Forest to encapsulate the HashMap that contains the "new" method that generates the tree structure

impl Forest {
    pub fn new(reader: BufReader<impl Read>) -> Result<Self, String> {
        let parser = parser::TreeParser::new();
        let mut stack = stack::NodeStack::new();
        let mut i = 0;
        let mut prev_level:u32 = 0;
        let mut current_tree_id = String::new();
        let mut forest = Forest { trees: HashMap::new() };
    
        for l in reader.lines() {
            i += 1;
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
                            let mut tree = Tree { nodes: vec!() };
                            tree.nodes.push(TreeNode {
                                content: String::from(&content),
                                level,
                                parent_position: None,
                                children: vec!()
                            });
                            forest.trees.insert(String::from(&current_tree_id), tree);
    
                            // Put node reference on stack
                            stack.push(stack::NodeStackContent::new(&content, level, 0));
                        }
                        else {
                            // Somebody's child node
                            if let Some(parent_node) = stack.pop_parent(level) {
                                println!("My parent is {}", parent_node.node);
                                
                                if let Some(tree) = forest.trees.get_mut(&current_tree_id) {
                                    // Put new node in the tree
                                    tree.nodes.push(TreeNode {
                                        content: String::from(&content),
                                        level,
                                        parent_position: Some(parent_node.tree_position),
                                        children: vec!()
                                    });
                                    let new_node_position = tree.nodes.len() as u32 - 1;
                                    // Attach node to parent
                                    if let Some(parent_node_real) = tree.nodes.get_mut(parent_node.tree_position as usize) {
                                        parent_node_real.children.push(new_node_position);
                                    }

                                    // Return parent reference node to stack
                                    stack.push(parent_node);
                                    // Push new node reference to stack
                                    stack.push(stack::NodeStackContent::new(&content, level, new_node_position));
                                }
                            }
                            else {
                                return Result::Err(format!("Couldn't find a parent at line {}", i));
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
    
        Result::Ok(forest)
    }    
}

/*
TODO:
- Genera un dict amb tantes claus com tree names a l'arxiu.
- Generate tree with default object or user defined. La callback de l'usr rep: node str, parent node i depth.
- Traversal iterator using various algorithms (https://towardsdatascience.com/4-types-of-tree-traversal-algorithms-d56328450846 , https://en.wikipedia.org/wiki/Tree_traversal).
- Find a specific node.
- Access a specific node by using a path.
- Allow using the BufReader directly to read data from the tree, instead of parsing and generating a model in mem. For very big trees.
*/