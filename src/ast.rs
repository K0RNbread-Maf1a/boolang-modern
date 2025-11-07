//! Abstract Syntax Tree definitions for Boolang

use serde::{Deserialize, Serialize};
use std::fmt;

/// Source location information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// Starting line number (1-indexed)
    pub line: usize,
    /// Starting column number (1-indexed)
    pub column: usize,
    /// Length in characters
    pub length: usize,
}

/// Program - root node of the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    /// Module declarations
    pub modules: Vec<Module>,
    /// Top-level statements
    pub statements: Vec<Statement>,
}

/// Module declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    /// Module name
    pub name: String,
    /// Module members
    pub members: Vec<Statement>,
    /// Source location
    pub span: Span,
}

/// Statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    /// Function definition
    FunctionDef(FunctionDef),
    /// Class definition
    ClassDef(ClassDef),
    /// Variable declaration
    VarDecl(VarDecl),
    /// Expression statement
    Expr(Expression),
    /// Return statement
    Return(Option<Expression>, Span),
    /// If statement
    If(IfStatement),
    /// While loop
    While(WhileLoop),
    /// For loop
    For(ForLoop),
    /// Import statement
    Import(ImportStatement),
}

/// Function definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDef {
    /// Function name
    pub name: String,
    /// Parameters
    pub parameters: Vec<Parameter>,
    /// Return type
    pub return_type: Option<Type>,
    /// Function body
    pub body: Vec<Statement>,
    /// Source location
    pub span: Span,
}

/// Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub type_annotation: Option<Type>,
    /// Default value
    pub default: Option<Expression>,
}

/// Class definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDef {
    /// Class name
    pub name: String,
    /// Base classes
    pub bases: Vec<String>,
    /// Class members
    pub members: Vec<Statement>,
    /// Source location
    pub span: Span,
}

/// Variable declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarDecl {
    /// Variable name
    pub name: String,
    /// Type annotation
    pub type_annotation: Option<Type>,
    /// Initial value
    pub initializer: Option<Expression>,
    /// Is mutable
    pub mutable: bool,
    /// Source location
    pub span: Span,
}

/// If statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfStatement {
    /// Condition
    pub condition: Expression,
    /// Then branch
    pub then_branch: Vec<Statement>,
    /// Else branch
    pub else_branch: Option<Vec<Statement>>,
    /// Source location
    pub span: Span,
}

/// While loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileLoop {
    /// Condition
    pub condition: Expression,
    /// Loop body
    pub body: Vec<Statement>,
    /// Source location
    pub span: Span,
}

/// For loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForLoop {
    /// Loop variable
    pub variable: String,
    /// Iterable expression
    pub iterable: Expression,
    /// Loop body
    pub body: Vec<Statement>,
    /// Source location
    pub span: Span,
}

/// Import statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportStatement {
    /// Module path
    pub path: Vec<String>,
    /// Imported items (None = import all)
    pub items: Option<Vec<String>>,
    /// Alias
    pub alias: Option<String>,
    /// Source location
    pub span: Span,
}

/// Expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    /// Literal value
    Literal(Literal),
    /// Variable reference
    Identifier(String, Span),
    /// Binary operation
    Binary(BinaryOp),
    /// Unary operation
    Unary(UnaryOp),
    /// Function call
    Call(CallExpr),
    /// Member access
    Member(MemberExpr),
    /// Array/list literal
    Array(Vec<Expression>, Span),
    /// Dictionary/map literal
    Dict(Vec<(Expression, Expression)>, Span),
    /// Lambda expression
    Lambda(LambdaExpr),
}

/// Literal value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    /// Integer
    Int(i64),
    /// Float
    Float(f64),
    /// String
    String(String),
    /// Boolean
    Bool(bool),
    /// Null/None
    Null,
}

/// Binary operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryOp {
    /// Left operand
    pub left: Box<Expression>,
    /// Operator
    pub op: BinaryOperator,
    /// Right operand
    pub right: Box<Expression>,
    /// Source location
    pub span: Span,
}

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperator {
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Modulo
    Mod,
    /// Equality
    Eq,
    /// Inequality
    Ne,
    /// Less than
    Lt,
    /// Less than or equal
    Le,
    /// Greater than
    Gt,
    /// Greater than or equal
    Ge,
    /// Logical AND
    And,
    /// Logical OR
    Or,
}

/// Unary operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryOp {
    /// Operator
    pub op: UnaryOperator,
    /// Operand
    pub operand: Box<Expression>,
    /// Source location
    pub span: Span,
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOperator {
    /// Negation
    Neg,
    /// Logical NOT
    Not,
}

/// Function call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallExpr {
    /// Function expression
    pub function: Box<Expression>,
    /// Arguments
    pub arguments: Vec<Expression>,
    /// Source location
    pub span: Span,
}

/// Member access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberExpr {
    /// Object expression
    pub object: Box<Expression>,
    /// Member name
    pub member: String,
    /// Source location
    pub span: Span,
}

/// Lambda expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LambdaExpr {
    /// Parameters
    pub parameters: Vec<Parameter>,
    /// Body
    pub body: Vec<Statement>,
    /// Source location
    pub span: Span,
}

/// Type annotation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    /// Integer
    Int,
    /// Float
    Float,
    /// String
    String,
    /// Boolean
    Bool,
    /// Void
    Void,
    /// Array
    Array(Box<Type>),
    /// Function type
    Function(Vec<Type>, Box<Type>),
    /// Custom type
    Custom(String),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::Bool => write!(f, "bool"),
            Type::Void => write!(f, "void"),
            Type::Array(inner) => write!(f, "{}[]", inner),
            Type::Function(params, ret) => {
                write!(f, "(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", ret)
            }
            Type::Custom(name) => write!(f, "{}", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_display() {
        assert_eq!(Type::Int.to_string(), "int");
        assert_eq!(Type::Array(Box::new(Type::String)).to_string(), "string[]");
    }

    #[test]
    fn test_span() {
        let span = Span {
            line: 1,
            column: 5,
            length: 10,
        };
        assert_eq!(span.line, 1);
        assert_eq!(span.column, 5);
    }
}
