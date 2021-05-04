use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use regex::Regex;

enum TreeStatement {
    TreeID(String),
    Node(String, u32),
    Comment,
    Empty,
    Invalid
}

struct TreeParser {
    tree_id_matcher: Regex,
    tree_id_finder: Regex,
    node_matcher: Regex,
    node_finder: Regex,
    node_level_finder: Regex,
    comment_matcher: Regex
}

impl TreeParser {
    fn new() -> Self {
        Self {
            tree_id_matcher: Regex::new(r"^\[[A-Za-z0-9_]+\]$").unwrap(),
            tree_id_finder: Regex::new(r"[A-Za-z0-9_]+").unwrap(),
            node_matcher: Regex::new(r"^(\+ )+[^\+].*$").unwrap(),
            node_finder: Regex::new(r"(\+ )+").unwrap(),
            node_level_finder: Regex::new(r"(\+ )").unwrap(),
            comment_matcher: Regex::new(r"^#.*+$").unwrap()
        }
    }

    fn parse_statement(&self, statement: &String) -> TreeStatement {
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

#[derive(Debug)]
struct NodeStack {
    buffer: Vec<NodeStackContent>
}

#[derive(Debug)]
struct NodeStackContent {
    node: String,
    level: u32
}

impl NodeStack {
    fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }

    fn push(&mut self, obj: NodeStackContent) {
        self.buffer.push(obj);
    }

    fn pop(&mut self, skip: u32) -> Option<NodeStackContent> {
        //remove "skip" elements and then pop
        if self.buffer.len() - skip as usize > 0 {
            self.buffer.truncate(self.buffer.len() - skip as usize);
        }
        self.buffer.pop()
    }

    fn pop_parent(&mut self, level: u32) -> Option<NodeStackContent> {
        //obtain data from stack until we get one node with a level lower than "level"
        loop {
            let n = self.pop(0);
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

    fn flush(&mut self) {
        self.buffer.truncate(0);
    }
/*
    fn top(&mut self) -> Option<&TreeNode<'a>> {
        self.buffer.last()
    }

    fn top_level(&mut self) -> Option<u32> {
        if let Some(ts) = self.top() {
            if let TreeStatement::Node(_, l) = ts {
                return Some(*l);
            }
        }
        None
    }
*/
}

impl NodeStackContent {
    fn new(node: &String, level: u32) -> Self {
        Self {
            node: String::from(node),
            level
        }
    }
}

#[derive(Debug)]
pub struct TreeNode<'a> {
    pub children: Vec<TreeNode<'a>>,
    pub level: u32,
    pub parent: Option<&'a TreeNode<'a>>,
    //TODO: make the content generic type
    pub content: String
}

//Data structure for level by level access. This struct makes reference to the TreeNode struct, that is the main structure to hold the nodes and the tree.
// only for Breadth-first trasverse
/*
#[derive(Debug)]
pub struct LevelTreeNode<'a> {
    pub nodes: Vec<Option<&'a TreeNode<'a>>>,
    pub level: u32
}
*/

pub fn build_tree<'a>(reader: BufReader<impl Read>) -> Result<HashMap<String, TreeNode<'a>>, String> {
    let parser =  TreeParser::new();
    let mut stack = NodeStack::new();
    let mut i = 0;
    let mut prev_level:u32 = 0;
    let mut root_node = TreeNode {
        children: vec!(),
        level: 0,
        parent: None,
        content: String::new()
    };
    let mut previous_node = NodeStackContent::new(&String::new(), 0);

    for l in reader.lines() {
        i += 1;
        println!("----------------");
        if let Ok(line) = l {
            let statement = parser.parse_statement(&line);
            match statement {
                TreeStatement::Invalid => return Result::Err(format!("Invalid statement at line {}", i)),
                TreeStatement::TreeID(tree_id) => {
                    println!("tree_id       {}", tree_id);
                    stack.flush();
                }
                TreeStatement::Node(content, level) => {
                    println!("node          {} (level: {})", content, level);

                    if level > prev_level + 1 {
                        return Result::Err(format!("Invalid node level at line {}", i));
                    }

                    if level == 1 {
                        //Root node
                        println!("Root node");
                        stack.push(NodeStackContent::new(&content, level));
                        root_node = TreeNode {
                            children: vec!(),
                            level: level,
                            parent: None,
                            content: content
                        };
                    }
                    else {
                        //let top_stack = stack.pop(0);
                        if let Some(parent_node) = stack.pop_parent(level) {
                            println!("My parent is {}", parent_node.node);
                            stack.push(parent_node);
                        }
                        else {
                            println!("Not found a parent");
                        }

                        stack.push(NodeStackContent::new(&content, level));
                    }

                    /*
                    let top_stack = stack.pop(0);

                    if let Some(top_stack_node) = top_stack {
                        println!("Somebody's child");

                        if top_stack_node.level == level {
                            //TODO: Siblings, do not get back to stack
                            println!("Siblings");
                            println!("My parent is {}", top_stack_node.node);
                        }
                        else if level < top_stack_node.level  {
                            //TODO: this branch ends
                            println!("Branch ends");
                            stack.push(top_stack_node);
                        }
                        else {
                            println!("Continue the branch");
                            println!("My parent is {}", top_stack_node.node);
                            stack.push(top_stack_node);
                        }
                        
                        stack.push(NodeStackContent::new(&content, level));

                        previous_node = NodeStackContent::new(&content, level);
                    }
                    else {
                        println!("Root node");
                        stack.push(NodeStackContent::new(&content, level));
                        root_node = TreeNode {
                            children: vec!(),
                            level: level,
                            parent: None,
                            content: content
                        };
                    }
                    */

                    prev_level = level;
                },
                _ => {}
            }
        }
        else {
           return Result::Err(format!("Could not read line at {}", i));
        }
    }

    println!("{:#?}", stack);

    //TODO: return the tree structure
    Result::Ok(HashMap::new())
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