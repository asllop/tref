use regex::Regex;

/// Statements of a TREF document.
pub enum TreeStatement {
    /// Tree ID statement, with tree name.
    TreeID(String),
    /// Node statement, with node content and level.
    Node(String, usize),
    /// Comment statement.
    Comment,
    /// Empty statement.
    Empty,
    /// Invalid statement.
    Invalid
}

/// Tree parser.
pub struct TreeParser {
    tree_id_matcher: Regex,
    tree_id_finder: Regex,
    node_matcher: Regex,
    node_finder: Regex,
    node_level_finder: Regex,
    comment_matcher: Regex
}

impl TreeParser {
    /// Create a new tree parser.
    pub fn new() -> Self {
        Self {
            tree_id_matcher: Regex::new(r"^\[[^\[\]]+\]$").unwrap(),
            tree_id_finder: Regex::new(r"[^\[\]]+").unwrap(),
            node_matcher: Regex::new(r"^(\+ )+[^\+ ].*$").unwrap(),
            node_finder: Regex::new(r"(\+ )+").unwrap(),
            node_level_finder: Regex::new(r"(\+ )").unwrap(),
            comment_matcher: Regex::new(r"^#.*+$").unwrap()
        }
    }

    /// Parse one statement.
    /// 
    /// # Arguments
    /// 
    /// * `statement` - Document line.
    /// 
    /// # Return
    /// 
    /// * A [`TreeStatement`] model.
    ///
    pub fn parse_statement(&self, statement: &str) -> TreeStatement {
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