//! Runtime module - Tree-walking interpreter

use crate::ast::*;
use crate::error::{BoolangError, Result};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
}

pub struct Runtime {
    variables: HashMap<String, Value>,
}

impl Runtime {
    pub fn new() -> Result<Self> {
        Ok(Self {
            variables: HashMap::new(),
        })
    }

    pub fn execute(&mut self, _bytecode: &[u8]) -> Result<()> {
        // Bytecode execution not yet implemented
        Ok(())
    }

    pub fn interpret(&mut self, program: &Program) -> Result<Option<Value>> {
        for statement in &program.statements {
            if let Some(val) = self.execute_statement(statement)? {
                return Ok(Some(val));
            }
        }
        Ok(None)
    }

    fn execute_statement(&mut self, stmt: &Statement) -> Result<Option<Value>> {
        match stmt {
            Statement::Expr(expr) => {
                self.eval_expression(expr)?;
                Ok(None)
            }
            Statement::VarDecl(var_decl) => {
                let value = if let Some(init) = &var_decl.initializer {
                    self.eval_expression(init)?
                } else {
                    Value::Null
                };
                self.variables.insert(var_decl.name.clone(), value);
                Ok(None)
            }
            Statement::Return(expr, _) => {
                let value = if let Some(e) = expr {
                    self.eval_expression(e)?
                } else {
                    Value::Null
                };
                Ok(Some(value))
            }
            _ => Ok(None),
        }
    }

    fn eval_expression(&mut self, expr: &Expression) -> Result<Value> {
        match expr {
            Expression::Literal(lit) => Ok(self.eval_literal(lit)),
            Expression::Identifier(name, _span) => {
                self.variables
                    .get(name)
                    .cloned()
                    .ok_or_else(|| BoolangError::runtime(format!("Undefined variable: {}", name)))
            }
            Expression::Binary(bin_op) => self.eval_binary(bin_op),
            Expression::Unary(un_op) => self.eval_unary(un_op),
            Expression::Call(call) => self.eval_call(call),
            _ => Err(BoolangError::runtime("Expression not yet implemented")),
        }
    }

    fn eval_literal(&self, lit: &Literal) -> Value {
        match lit {
            Literal::Int(i) => Value::Int(*i),
            Literal::Float(f) => Value::Float(*f),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Bool(b) => Value::Bool(*b),
            Literal::Null => Value::Null,
        }
    }

    fn eval_binary(&mut self, bin_op: &BinaryOp) -> Result<Value> {
        let left = self.eval_expression(&bin_op.left)?;
        let right = self.eval_expression(&bin_op.right)?;

        match (left, right) {
            (Value::Int(l), Value::Int(r)) => match bin_op.op {
                BinaryOperator::Add => Ok(Value::Int(l + r)),
                BinaryOperator::Sub => Ok(Value::Int(l - r)),
                BinaryOperator::Mul => Ok(Value::Int(l * r)),
                BinaryOperator::Div => Ok(Value::Int(l / r)),
                BinaryOperator::Mod => Ok(Value::Int(l % r)),
                BinaryOperator::Eq => Ok(Value::Bool(l == r)),
                BinaryOperator::Ne => Ok(Value::Bool(l != r)),
                BinaryOperator::Lt => Ok(Value::Bool(l < r)),
                BinaryOperator::Le => Ok(Value::Bool(l <= r)),
                BinaryOperator::Gt => Ok(Value::Bool(l > r)),
                BinaryOperator::Ge => Ok(Value::Bool(l >= r)),
                _ => Err(BoolangError::runtime("Invalid operation on integers")),
            },
            (Value::Float(l), Value::Float(r)) => match bin_op.op {
                BinaryOperator::Add => Ok(Value::Float(l + r)),
                BinaryOperator::Sub => Ok(Value::Float(l - r)),
                BinaryOperator::Mul => Ok(Value::Float(l * r)),
                BinaryOperator::Div => Ok(Value::Float(l / r)),
                BinaryOperator::Eq => Ok(Value::Bool(l == r)),
                BinaryOperator::Ne => Ok(Value::Bool(l != r)),
                BinaryOperator::Lt => Ok(Value::Bool(l < r)),
                BinaryOperator::Le => Ok(Value::Bool(l <= r)),
                BinaryOperator::Gt => Ok(Value::Bool(l > r)),
                BinaryOperator::Ge => Ok(Value::Bool(l >= r)),
                _ => Err(BoolangError::runtime("Invalid operation on floats")),
            },
            (Value::String(l), Value::String(r)) => match bin_op.op {
                BinaryOperator::Add => Ok(Value::String(format!("{}{}", l, r))),
                BinaryOperator::Eq => Ok(Value::Bool(l == r)),
                BinaryOperator::Ne => Ok(Value::Bool(l != r)),
                _ => Err(BoolangError::runtime("Invalid operation on strings")),
            },
            (Value::Bool(l), Value::Bool(r)) => match bin_op.op {
                BinaryOperator::And => Ok(Value::Bool(l && r)),
                BinaryOperator::Or => Ok(Value::Bool(l || r)),
                BinaryOperator::Eq => Ok(Value::Bool(l == r)),
                BinaryOperator::Ne => Ok(Value::Bool(l != r)),
                _ => Err(BoolangError::runtime("Invalid operation on booleans")),
            },
            _ => Err(BoolangError::runtime("Type mismatch in binary operation")),
        }
    }

    fn eval_unary(&mut self, un_op: &UnaryOp) -> Result<Value> {
        let operand = self.eval_expression(&un_op.operand)?;

        match (un_op.op, operand) {
            (UnaryOperator::Neg, Value::Int(i)) => Ok(Value::Int(-i)),
            (UnaryOperator::Neg, Value::Float(f)) => Ok(Value::Float(-f)),
            (UnaryOperator::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
            _ => Err(BoolangError::runtime("Invalid unary operation")),
        }
    }

    fn eval_call(&mut self, call: &CallExpr) -> Result<Value> {
        // Handle built-in functions
        if let Expression::Identifier(name, _) = &*call.function {
            if name == "print" {
                for arg in &call.arguments {
                    let val = self.eval_expression(arg)?;
                    println!("{}", val);
                }
                return Ok(Value::Null);
            }
        }

        Err(BoolangError::runtime("Function not found"))
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
