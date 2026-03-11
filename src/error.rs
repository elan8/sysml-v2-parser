//! Parse error types for SysML v2 parser.
//!
//! All line and column values are **1-based**. Use [`ParseError::to_lsp_range`] for
//! 0-based (line, character) ranges as used by the Language Server Protocol.

/// Severity of a parse diagnostic (for language server integration).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
}

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
    /// Optional length of the error span in bytes (for LSP range end).
    pub length: Option<usize>,
    /// Severity (defaults to Error when not set).
    pub severity: Option<DiagnosticSeverity>,
    /// Optional code for quick fixes or documentation (e.g. "expected_keyword").
    pub code: Option<String>,
}

impl ParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            offset: None,
            line: None,
            column: None,
            length: None,
            severity: None,
            code: None,
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

    pub fn with_length(mut self, length: usize) -> Self {
        self.length = Some(length);
        self
    }

    pub fn with_severity(mut self, severity: DiagnosticSeverity) -> Self {
        self.severity = Some(severity);
        self
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// LSP uses 0-based line and 0-based character. Returns (start_line, start_character, end_line, end_character).
    /// Returns `None` if position is unknown.
    pub fn to_lsp_range(&self) -> Option<(u32, u32, u32, u32)> {
        let (line, column) = (self.line?, self.column?);
        let len = self.length.unwrap_or(1);
        let start_line = line.saturating_sub(1);
        let start_char = column.saturating_sub(1);
        let end_line = start_line;
        let end_char = start_char.saturating_add(len);
        Some((start_line, start_char as u32, end_line, end_char as u32))
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
