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
pub struct TreeNode<'a> {
    pub children: Vec<TreeNode<'a>>,
    pub parent: Option<&'a TreeNode<'a>>,
    //TODO: make the content generic type
    pub content: String
}

pub fn parse_tree<'a>(reader: BufReader<impl Read>) -> Result<HashMap<String, TreeNode<'a>>, String> {
    let parser =  TreeParser::new();
    let mut i = 0;
    for l in reader.lines() {
        i += 1;
        if let Ok(line) = l {
            //TODO: build the tree structure
            match parser.parse_statement(&line) {
                TreeStatement::TreeID(s)    => println!("tree_id       {}", s),
                TreeStatement::Node(s, l)   => println!("node          {} ({})", s, l),
                TreeStatement::Invalid      => return Result::Err(format!("Invalid statement at line {}", i)),
                _ => {}
            }
        }
        else {
           return Result::Err(format!("Could not read line at {}", i));
        }
    }
    //TODO: return the tree structure
    Result::Ok(HashMap::new())
}

/*
TODO:
- Genera un dict amb tantes claus com tree names a l'arxiu.
- Generate tree with default object or user defined. La callback de l'usr rep: node str, parent node i depth.
- Traversal iterator using DFS and BFS algorithms (https://towardsdatascience.com/4-types-of-tree-traversal-algorithms-d56328450846).
*/