use boolang_modern::{Compiler, CompilerOptions, Runtime};

#[test]
fn test_simple_arithmetic() {
    let source = "x = 10 + 5";
    
    let compiler = Compiler::new(CompilerOptions::default());
    let result = compiler.compile(source).unwrap();
    
    let mut runtime = Runtime::new().unwrap();
    runtime.interpret(&result.program).unwrap();
}

#[test]
fn test_print_statement() {
    let source = r#"print("Hello, World!")"#;
    
    let compiler = Compiler::new(CompilerOptions::default());
    let result = compiler.compile(source).unwrap();
    
    let mut runtime = Runtime::new().unwrap();
    runtime.interpret(&result.program).unwrap();
}

#[test]
fn test_variable_and_arithmetic() {
    let source = "x = 42\nprint(x)";
    
    let compiler = Compiler::new(CompilerOptions::default());
    let result = compiler.compile(source).unwrap();
    
    let mut runtime = Runtime::new().unwrap();
    runtime.interpret(&result.program).unwrap();
}

#[test]
fn test_expression_arithmetic() {
    let source = "print(10 + 5 * 2)";
    
    let compiler = Compiler::new(CompilerOptions::default());
    let result = compiler.compile(source).unwrap();
    
    let mut runtime = Runtime::new().unwrap();
    runtime.interpret(&result.program).unwrap();
}
