//! Code generation module

use crate::ast::Program;
use crate::error::Result;

pub struct CodeGenerator {
    // Codegen state
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, _program: &Program) -> Result<Vec<u8>> {
        // TODO: Implement code generation
        Ok(vec![])
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}