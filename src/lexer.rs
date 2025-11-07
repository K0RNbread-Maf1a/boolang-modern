//! Lexer module - Tokenizes Boolang source code

use crate::error::{BoolangError, Result};

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace_and_comments();
            
            if self.is_at_end() {
                break;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            line: self.line,
            column: self.column,
        });

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        let ch = self.current();

        let kind = match ch {
            // Single character tokens
            '(' => { self.advance(); TokenKind::LeftParen }
            ')' => { self.advance(); TokenKind::RightParen }
            '{' => { self.advance(); TokenKind::LeftBrace }
            '}' => { self.advance(); TokenKind::RightBrace }
            '[' => { self.advance(); TokenKind::LeftBracket }
            ']' => { self.advance(); TokenKind::RightBracket }
            ',' => { self.advance(); TokenKind::Comma }
            ':' => { self.advance(); TokenKind::Colon }
            ';' => { self.advance(); TokenKind::Semicolon }
            '+' => { self.advance(); TokenKind::Plus }
            '-' => { self.advance(); TokenKind::Minus }
            '*' => { self.advance(); TokenKind::Star }
            '/' => { self.advance(); TokenKind::Slash }
            '%' => { self.advance(); TokenKind::Percent }
            
            // Two character tokens
            '=' => {
                self.advance();
                if self.match_char('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            '!' => {
                self.advance();
                if self.match_char('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            '<' => {
                self.advance();
                if self.match_char('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '>' => {
                self.advance();
                if self.match_char('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            
            // String literals
            '"' | '\'' => return self.string_literal(ch),
            
            // Numbers
            '0'..='9' => return self.number(),
            
            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => return self.identifier(),
            
            _ => {
                return Err(BoolangError::lexer(
                    self.line,
                    self.column,
                    format!("Unexpected character: {}", ch),
                ));
            }
        };

        Ok(Token {
            kind,
            lexeme: String::new(),
            line: start_line,
            column: start_column,
        })
    }

    fn identifier(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        let start = self.position;

        while !self.is_at_end() && (self.current().is_alphanumeric() || self.current() == '_') {
            self.advance();
        }

        let lexeme: String = self.source[start..self.position].iter().collect();
        let kind = self.keyword_or_identifier(&lexeme);

        Ok(Token {
            kind,
            lexeme,
            line: start_line,
            column: start_column,
        })
    }

    fn keyword_or_identifier(&self, text: &str) -> TokenKind {
        match text {
            "def" => TokenKind::Def,
            "class" => TokenKind::Class,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "elif" => TokenKind::Elif,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "return" => TokenKind::Return,
            "import" => TokenKind::Import,
            "from" => TokenKind::From,
            "as" => TokenKind::As,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            "print" => TokenKind::Print,
            _ => TokenKind::Identifier,
        }
    }

    fn number(&mut self) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        let start = self.position;

        while !self.is_at_end() && self.current().is_ascii_digit() {
            self.advance();
        }

        let mut kind = TokenKind::Integer;

        // Check for decimal point
        if !self.is_at_end() && self.current() == '.' && self.peek().map_or(false, |c| c.is_ascii_digit()) {
            kind = TokenKind::Float;
            self.advance(); // consume '.'
            while !self.is_at_end() && self.current().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme: String = self.source[start..self.position].iter().collect();

        Ok(Token {
            kind,
            lexeme,
            line: start_line,
            column: start_column,
        })
    }

    fn string_literal(&mut self, quote: char) -> Result<Token> {
        let start_line = self.line;
        let start_column = self.column;
        self.advance(); // consume opening quote

        let start = self.position;

        while !self.is_at_end() && self.current() != quote {
            if self.current() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(BoolangError::lexer(
                self.line,
                self.column,
                "Unterminated string",
            ));
        }

        let lexeme: String = self.source[start..self.position].iter().collect();
        self.advance(); // consume closing quote

        Ok(Token {
            kind: TokenKind::String,
            lexeme,
            line: start_line,
            column: start_column,
        })
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            if self.is_at_end() {
                break;
            }

            match self.current() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                    self.advance();
                }
                '#' => {
                    // Line comment
                    while !self.is_at_end() && self.current() != '\n' {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn current(&self) -> char {
        self.source[self.position]
    }

    fn peek(&self) -> Option<char> {
        if self.position + 1 < self.source.len() {
            Some(self.source[self.position + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
            self.column += 1;
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.current() != expected {
            false
        } else {
            self.advance();
            true
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Literals
    Identifier,
    Integer,
    Float,
    String,
    True,
    False,
    Null,

    // Keywords
    Def,
    Class,
    If,
    Else,
    Elif,
    While,
    For,
    In,
    Return,
    Import,
    From,
    As,
    And,
    Or,
    Not,
    Print,

    // Symbols
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Semicolon,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Special
    Eof,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("x = 42".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].lexeme, "x");
        assert_eq!(tokens[1].kind, TokenKind::Equal);
        assert_eq!(tokens[2].kind, TokenKind::Integer);
        assert_eq!(tokens[2].lexeme, "42");
    }

    #[test]
    fn test_string_literal() {
        let mut lexer = Lexer::new("\"hello\"".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::String);
        assert_eq!(tokens[0].lexeme, "hello");
    }

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("def if while".to_string());
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].kind, TokenKind::Def);
        assert_eq!(tokens[1].kind, TokenKind::If);
        assert_eq!(tokens[2].kind, TokenKind::While);
    }
}
