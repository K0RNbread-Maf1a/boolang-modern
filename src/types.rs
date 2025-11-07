//! Type system module

use crate::ast::Type;
use crate::error::Result;

pub struct TypeChecker {
    // Type checking state
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check(&self, _program: &crate::ast::Program) -> Result<()> {
        // TODO: Implement type checking
        Ok(())
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}