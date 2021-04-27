use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

struct TreeParser {
    tree_name_regex: Regex,
    node_regex: Regex,
    comment_regex: Regex
}

impl TreeParser {
    fn new() -> TreeParser {
        Self {
            tree_name_regex: Regex::new(r"^\[[A-Za-z0-9_]+\]$").unwrap(),
            node_regex: Regex::new(r"^(\+ )+[^\+].*$").unwrap(),
            comment_regex: Regex::new(r"^#.*+$").unwrap()
        }
    }
}

enum TreeStatement {
    TreeName(String),
    Node(String),
    Comment(String),
    Empty,
    Invalid(String)
}

impl TreeStatement {
    fn new(statement: &String, parser: &TreeParser) -> TreeStatement {
        if parser.node_regex.is_match(statement) {
            TreeStatement::Node(String::from(statement))
        }
        else if parser.tree_name_regex.is_match(statement) {
            TreeStatement::TreeName(String::from(statement))
        }
        else if parser.comment_regex.is_match(statement) {
            TreeStatement::Comment(String::from(statement))
        }
        else {
            if statement.len() == 0 {
                TreeStatement::Empty
            }
            else {
                TreeStatement::Invalid(String::from(statement))
            }
        }
    }
}

fn main() {
    let parser = TreeParser::new();
    if let Ok(file) = File::open("file.tdf") {
        let reader = BufReader::new(file);
        for l in reader.lines() {
            if let Ok(line) = l {
                match TreeStatement::new(&line, &parser) {
                    TreeStatement::TreeName(s)  => println!("tree name     {}", s),
                    TreeStatement::Node(s)      => println!("node          {}", s),
                    TreeStatement::Comment(s)   => println!("comment       {}", s),
                    TreeStatement::Empty        => println!("empty"),
                    TreeStatement::Invalid(s)   => println!("invalid       {}", s),
                }
            }
        }
    }

    println!("EOF.");
}