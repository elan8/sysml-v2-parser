//! Parse error types for SysML v2 parser.

/// Error returned when parsing fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// Human-readable description of the error.
    pub message: String,
    /// Optional byte offset in the input where the error occurred.
    pub offset: Option<usize>,
    /// Optional line number (1-based).
    pub line: Option<u32>,
    /// Optional column (1-based).
    pub column: Option<usize>,
}

impl ParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            offset: None,
            line: None,
            column: None,
        }
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set offset, line, and column for error location.
    pub fn with_location(mut self, offset: usize, line: u32, column: usize) -> Self {
        self.offset = Some(offset);
        self.line = Some(line);
        self.column = Some(column);
        self
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.offset, self.line, self.column) {
            (Some(off), Some(line), Some(col)) => {
                write!(f, "{} at line {}, column {} (offset {})", self.message, line, col, off)
            }
            (Some(off), _, _) => write!(f, "{} at offset {}", self.message, off),
            _ => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for ParseError {}
