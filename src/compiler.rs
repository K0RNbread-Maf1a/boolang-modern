//! Compiler module

use crate::ast::Program;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerOptions {
    pub optimize: bool,
    pub target: CompileTarget,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            optimize: true,
            target: CompileTarget::Bytecode,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompileTarget {
    Bytecode,
    #[cfg(feature = "dotnet")]
    DotNet,
    #[cfg(feature = "jvm")]
    Jvm,
    #[cfg(feature = "llvm")]
    Native,
}

pub struct Compiler {
    options: CompilerOptions,
}

impl Compiler {
    pub fn new(options: CompilerOptions) -> Self {
        Self { options }
    }

    pub fn compile(&self, source: &str) -> Result<CompileResult> {
        // Lex
        let mut lexer = crate::lexer::Lexer::new(source.to_string());
        let tokens = lexer.tokenize()?;

        // Parse
        let mut parser = crate::parser::Parser::new(tokens);
        let program = parser.parse()?;

        // Generate bytecode (stub for now)
        let bytecode = vec![];

        Ok(CompileResult {
            bytecode,
            program,
        })
    }
}

pub struct CompileResult {
    pub bytecode: Vec<u8>,
    pub program: Program,
}