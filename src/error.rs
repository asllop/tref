use std::error::Error;
use std::fmt;

#[derive(Debug)]
/// Parse TREF document error.
pub struct ParseTreeError {
    message: String,
    line: usize
}

impl ParseTreeError {
    /// Create new parse tree error.
    /// 
    /// # Arguments
    /// 
    /// * `msg` - Error message.
    /// * `line` - Document line where the error hapened.
    /// 
    /// # Return
    /// 
    /// * An error model.
    ///
    pub fn new(msg: &str, line: usize) -> Self {
        ParseTreeError {
            message: String::from(msg),
            line
        }
    }

    /// Get error line.
    /// 
    /// # Return
    /// 
    /// * Line.
    ///
    pub fn line(&self) -> usize {
        self.line
    }
}

impl fmt::Display for ParseTreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}` at line {}", self.message, self.line + 1)
    }
}

impl Error for ParseTreeError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug)]
/// Serializer error.
pub struct SerializeTreeError {
    message: String,
    line: usize,
    statement: Option<String>
}

impl SerializeTreeError {
    /// Create new serializer error.
    /// 
    /// # Arguments
    /// 
    /// * `msg` - Error message.
    /// * `line` - File line.
    /// * `statement` - Statement that caused the problem.
    /// 
    /// # Return
    /// 
    /// * An error model.
    ///
    pub fn new(message: &str, line: usize, statement: Option<String>) -> Self {
        SerializeTreeError {
            message: String::from(message),
            line,
            statement
        }
    }

    /// Get error line.
    /// 
    /// # Return
    /// 
    /// * Line.
    ///
    pub fn line(&self) -> usize {
        self.line
    }

    /// Get statement that caused the error.
    /// 
    /// # Return
    /// 
    /// * Statement.
    ///
    pub fn statement(&self) -> &Option<String> {
        &self.statement
    }
}

impl fmt::Display for SerializeTreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(statement) = &self.statement {
            write!(f, "`{}` at line {} with statement {}", self.message, self.line, statement)
        }
        else {
            write!(f, "`{}` at line {}", self.message, self.line)
        }
    }
}

impl Error for SerializeTreeError {
    fn description(&self) -> &str {
        &self.message
    }
}