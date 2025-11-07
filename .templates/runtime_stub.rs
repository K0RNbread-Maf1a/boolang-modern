//! Runtime module

use crate::error::Result;

pub struct Runtime {
    // Runtime state
}

impl Runtime {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn execute(&mut self, bytecode: &[u8]) -> Result<()> {
        // TODO: Implement
        Ok(())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new().unwrap()
    }
}