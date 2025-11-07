// Code generation backends

use crate::typechecker::TypedCompilationUnit;
use anyhow::Result;
use std::path::Path;

#[cfg(feature = "dotnet")]
pub mod dotnet;

#[cfg(feature = "jvm")]
pub mod jvm;

#[cfg(feature = "llvm")]
pub mod llvm;

pub trait CodegenBackend {
    fn generate(&mut self, unit: &TypedCompilationUnit) -> Result<Vec<u8>>;
    fn write_output(&self, output: &[u8], path: &Path) -> Result<()>;
}

pub struct CodeGenerator {
    backend: Box<dyn CodegenBackend>,
}

impl CodeGenerator {
    pub fn new(backend: Box<dyn CodegenBackend>) -> Self {
        Self { backend }
    }

    pub fn generate(&mut self, unit: &TypedCompilationUnit, output_path: &Path) -> Result<()> {
        let output = self.backend.generate(unit)?;
        self.backend.write_output(&output, output_path)?;
        Ok(())
    }
}

#[cfg(feature = "dotnet")]
pub fn create_dotnet_backend() -> Box<dyn CodegenBackend> {
    Box::new(dotnet::DotNetBackend::new())
}

#[cfg(feature = "jvm")]
pub fn create_jvm_backend() -> Box<dyn CodegenBackend> {
    Box::new(jvm::JvmBackend::new())
}

#[cfg(feature = "llvm")]
pub fn create_llvm_backend() -> Box<dyn CodegenBackend> {
    Box::new(llvm::LlvmBackend::new())
}
