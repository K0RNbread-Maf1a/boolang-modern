//! Error types for Boolang

use std::fmt;
use thiserror::Error;

/// Result type for Boolang operations
pub type Result<T> = std::result::Result<T, BoolangError>;

/// Main error type for Boolang
#[derive(Debug, Error)]
pub enum BoolangError {
    /// Lexical analysis error
    #[error("Lexer error at line {line}, column {column}: {message}")]
    LexerError {
        /// Line number where error occurred
        line: usize,
        /// Column number where error occurred
        column: usize,
        /// Error message
        message: String,
    },

    /// Parser error
    #[error("Parser error at line {line}: {message}")]
    ParserError {
        /// Line number where error occurred
        line: usize,
        /// Error message
        message: String,
    },

    /// Type checking error
    #[error("Type error at line {line}: {message}")]
    TypeError {
        /// Line number where error occurred
        line: usize,
        /// Error message
        message: String,
    },

    /// Code generation error
    #[error("Codegen error: {0}")]
    CodegenError(String),

    /// Runtime error
    #[error("Runtime error: {0}")]
    RuntimeError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Initialization error
    #[error("Initialization error: {0}")]
    InitError(String),

    /// FFI error
    #[error("FFI error: {0}")]
    FfiError(String),

    /// Feature not implemented
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}

impl BoolangError {
    /// Create a lexer error
    pub fn lexer(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::LexerError {
            line,
            column,
            message: message.into(),
        }
    }

    /// Create a parser error
    pub fn parser(line: usize, message: impl Into<String>) -> Self {
        Self::ParserError {
            line,
            message: message.into(),
        }
    }

    /// Create a type error
    pub fn type_error(line: usize, message: impl Into<String>) -> Self {
        Self::TypeError {
            line,
            message: message.into(),
        }
    }

    /// Create a codegen error
    pub fn codegen(message: impl Into<String>) -> Self {
        Self::CodegenError(message.into())
    }

    /// Create a runtime error
    pub fn runtime(message: impl Into<String>) -> Self {
        Self::RuntimeError(message.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = BoolangError::lexer(10, 5, "Unexpected token");
        assert!(err.to_string().contains("line 10"));
        assert!(err.to_string().contains("column 5"));
    }

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let boo_err: BoolangError = io_err.into();
        assert!(matches!(boo_err, BoolangError::IoError(_)));
    }
}
