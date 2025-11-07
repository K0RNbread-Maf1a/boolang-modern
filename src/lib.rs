//! # Boolang Modern
//!
//! A modern, multi-platform implementation of the Boo programming language.
//!
//! ## Features
//!
//! - **Multi-target compilation**: Compile to .NET IL, JVM bytecode, Lua, Python, JavaScript, or native code
//! - **LSP Support**: Full Language Server Protocol implementation for IDE integration
//! - **FFI Support**: C-compatible FFI for integration with other languages
//! - **Async/Await**: First-class support for asynchronous programming
//! - **Type Inference**: Strong static typing with intelligent type inference
//!
//! ## Example
//!
//! ```rust
//! use boolang_modern::{Compiler, CompilerOptions};
//!
//! let source = r#"
//!     def hello(name: string):
//!         print("Hello, ${name}!")
//!     
//!     hello("World")
//! "#;
//!
//! let options = CompilerOptions::default();
//! let compiler = Compiler::new(options);
//! let result = compiler.compile(source)?;
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod ast;
pub mod codegen;
pub mod compiler;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod types;

#[cfg(feature = "dotnet")]
pub mod dotnet;

#[cfg(feature = "jvm")]
pub mod jvm;

#[cfg(feature = "lua")]
pub mod lua_backend;

#[cfg(feature = "python")]
pub mod python;

#[cfg(feature = "javascript")]
pub mod javascript;

#[cfg(feature = "llvm")]
pub mod llvm_backend;

// FFI exports
pub mod ffi;

// Re-exports
pub use compiler::{Compiler, CompilerOptions, CompileResult};
pub use error::{BoolangError, Result};
pub use runtime::Runtime;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the Boolang runtime
///
/// This should be called once before using any Boolang functionality.
pub fn init() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .try_init()
        .map_err(|e| BoolangError::InitError(e.to_string()))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_init() {
        // Init can be called multiple times safely
        let _ = init();
    }
}
