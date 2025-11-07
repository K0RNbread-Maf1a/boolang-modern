//! Parser module - Recursive descent parser for Boolang

use crate::ast::*;
use crate::error::{BoolangError, Result};
use crate::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement()?);
        }

        Ok(Program {
            modules: vec![],
            statements,
        })
    }

    fn statement(&mut self) -> Result<Statement> {
        match self.peek().kind {
            TokenKind::Print => self.print_statement(),
            TokenKind::Return => self.return_statement(),
            _ => {
                // Try variable declaration or expression
                if self.is_variable_declaration() {
                    self.variable_declaration()
                } else {
                    let expr = self.expression()?;
                    Ok(Statement::Expr(expr))
                }
            }
        }
    }

    fn variable_declaration(&mut self) -> Result<Statement> {
        let start = self.peek().clone();
        let name = self.consume_identifier()?;

        self.consume(TokenKind::Equal)?;
        let initializer = Some(self.expression()?);

        Ok(Statement::VarDecl(VarDecl {
            name,
            type_annotation: None,
            initializer,
            mutable: true,
            span: Span {
                line: start.line,
                column: start.column,
                length: 0,
            },
        }))
    }

    fn return_statement(&mut self) -> Result<Statement> {
        let start = self.peek().clone();
        self.consume(TokenKind::Return)?;

        let value = if !self.is_at_end() && !self.check(TokenKind::Eof) {
            Some(self.expression()?)
        } else {
            None
        };

        Ok(Statement::Return(
            value,
            Span {
                line: start.line,
                column: start.column,
                length: 0,
            },
        ))
    }

    fn print_statement(&mut self) -> Result<Statement> {
        self.consume(TokenKind::Print)?;
        self.consume(TokenKind::LeftParen)?;
        let expr = self.expression()?;
        self.consume(TokenKind::RightParen)?;

        // Convert to expression statement (print call)
        Ok(Statement::Expr(Expression::Call(CallExpr {
            function: Box::new(Expression::Identifier(
                "print".to_string(),
                Span { line: 0, column: 0, length: 0 },
            )),
            arguments: vec![expr],
            span: Span { line: 0, column: 0, length: 0 },
        })))
    }

    fn expression(&mut self) -> Result<Expression> {
        self.logical_or()
    }

    fn logical_or(&mut self) -> Result<Expression> {
        let mut expr = self.logical_and()?;

        while self.match_token(TokenKind::Or) {
            let start = self.previous().clone();
            let right = self.logical_and()?;
            expr = Expression::Binary(BinaryOp {
                left: Box::new(expr),
                op: BinaryOperator::Or,
                right: Box::new(right),
                span: Span {
                    line: start.line,
                    column: start.column,
                    length: 0,
                },
            });
        }

        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expression> {
        let mut expr = self.equality()?;

        while self.match_token(TokenKind::And) {
            let start = self.previous().clone();
            let right = self.equality()?;
            expr = Expression::Binary(BinaryOp {
                left: Box::new(expr),
                op: BinaryOperator::And,
                right: Box::new(right),
                span: Span {
                    line: start.line,
                    column: start.column,
                    length: 0,
                },
            });
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.comparison()?;

        while let Some(op) = self.match_tokens(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
            let start = self.previous().clone();
            let right = self.comparison()?;
            let operator = match op {
                TokenKind::EqualEqual => BinaryOperator::Eq,
                TokenKind::BangEqual => BinaryOperator::Ne,
                _ => unreachable!(),
            };

            expr = Expression::Binary(BinaryOp {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
                span: Span {
                    line: start.line,
                    column: start.column,
                    length: 0,
                },
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression> {
        let mut expr = self.term()?;

        while let Some(op) = self.match_tokens(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let start = self.previous().clone();
            let right = self.term()?;
            let operator = match op {
                TokenKind::Greater => BinaryOperator::Gt,
                TokenKind::GreaterEqual => BinaryOperator::Ge,
                TokenKind::Less => BinaryOperator::Lt,
                TokenKind::LessEqual => BinaryOperator::Le,
                _ => unreachable!(),
            };

            expr = Expression::Binary(BinaryOp {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
                span: Span {
                    line: start.line,
                    column: start.column,
                    length: 0,
                },
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression> {
        let mut expr = self.factor()?;

        while let Some(op) = self.match_tokens(&[TokenKind::Plus, TokenKind::Minus]) {
            let start = self.previous().clone();
            let right = self.factor()?;
            let operator = match op {
                TokenKind::Plus => BinaryOperator::Add,
                TokenKind::Minus => BinaryOperator::Sub,
                _ => unreachable!(),
            };

            expr = Expression::Binary(BinaryOp {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
                span: Span {
                    line: start.line,
                    column: start.column,
                    length: 0,
                },
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.unary()?;

        while let Some(op) = self.match_tokens(&[TokenKind::Star, TokenKind::Slash, TokenKind::Percent]) {
            let start = self.previous().clone();
            let right = self.unary()?;
            let operator = match op {
                TokenKind::Star => BinaryOperator::Mul,
                TokenKind::Slash => BinaryOperator::Div,
                TokenKind::Percent => BinaryOperator::Mod,
                _ => unreachable!(),
            };

            expr = Expression::Binary(BinaryOp {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
                span: Span {
                    line: start.line,
                    column: start.column,
                    length: 0,
                },
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression> {
        if let Some(op) = self.match_tokens(&[TokenKind::Bang, TokenKind::Minus, TokenKind::Not]) {
            let start = self.previous().clone();
            let operand = self.unary()?;
            let operator = match op {
                TokenKind::Bang | TokenKind::Not => UnaryOperator::Not,
                TokenKind::Minus => UnaryOperator::Neg,
                _ => unreachable!(),
            };

            return Ok(Expression::Unary(UnaryOp {
                op: operator,
                operand: Box::new(operand),
                span: Span {
                    line: start.line,
                    column: start.column,
                    length: 0,
                },
            }));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expression> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(TokenKind::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression> {
        let mut arguments = Vec::new();

        if !self.check(TokenKind::RightParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenKind::RightParen)?;

        Ok(Expression::Call(CallExpr {
            function: Box::new(callee),
            arguments,
            span: Span { line: 0, column: 0, length: 0 },
        }))
    }

    fn primary(&mut self) -> Result<Expression> {
        let token = self.advance();

        match token.kind {
            TokenKind::True => Ok(Expression::Literal(Literal::Bool(true))),
            TokenKind::False => Ok(Expression::Literal(Literal::Bool(false))),
            TokenKind::Null => Ok(Expression::Literal(Literal::Null)),
            TokenKind::Integer => {
                let value = token.lexeme.parse::<i64>()
                    .map_err(|_| BoolangError::parser(token.line, "Invalid integer"))?;
                Ok(Expression::Literal(Literal::Int(value)))
            }
            TokenKind::Float => {
                let value = token.lexeme.parse::<f64>()
                    .map_err(|_| BoolangError::parser(token.line, "Invalid float"))?;
                Ok(Expression::Literal(Literal::Float(value)))
            }
            TokenKind::String => Ok(Expression::Literal(Literal::String(token.lexeme.clone()))),
            TokenKind::Identifier => Ok(Expression::Identifier(
                token.lexeme.clone(),
                Span {
                    line: token.line,
                    column: token.column,
                    length: token.lexeme.len(),
                },
            )),
            TokenKind::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenKind::RightParen)?;
                Ok(expr)
            }
            _ => Err(BoolangError::parser(token.line, format!("Unexpected token: {:?}", token.kind))),
        }
    }

    fn is_variable_declaration(&self) -> bool {
        if !self.check(TokenKind::Identifier) {
            return false;
        }

        // Look ahead to see if next token is =
        if self.current + 1 < self.tokens.len() {
            matches!(self.tokens[self.current + 1].kind, TokenKind::Equal)
        } else {
            false
        }
    }

    // Helper methods
    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_tokens(&mut self, kinds: &[TokenKind]) -> Option<TokenKind> {
        for kind in kinds {
            if self.check(*kind) {
                let matched = *kind;
                self.advance();
                return Some(matched);
            }
        }
        None
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: TokenKind) -> Result<()> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            Err(BoolangError::parser(
                self.peek().line,
                format!("Expected {:?}, got {:?}", kind, self.peek().kind),
            ))
        }
    }

    fn consume_identifier(&mut self) -> Result<String> {
        if self.check(TokenKind::Identifier) {
            let token = self.advance();
            Ok(token.lexeme.clone())
        } else {
            Err(BoolangError::parser(
                self.peek().line,
                "Expected identifier",
            ))
        }
    }
}
