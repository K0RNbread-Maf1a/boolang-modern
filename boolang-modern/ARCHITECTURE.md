# BooLang Modern - Architecture

## System Overview

BooLang Modern is a multi-platform, high-performance language implementation that combines:
- A Rust-based compiler core for performance
- Multiple compilation targets (.NET, JVM, Android, Native, WASM)
- Dynamic runtime integration (Lua, Python, JavaScript, Kotlin)
- Language Server Protocol (LSP) for IDE support

## Core Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Source Code (.boo)                           │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                  Lexer (Indentation-Aware)                       │
│  • Tokenization                                                  │
│  • INDENT/DEDENT generation                                      │
│  • Comment stripping                                             │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Parser (ANTLR4)                                │
│  • Syntax analysis                                               │
│  • Parse tree generation                                         │
│  • Error recovery                                                │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                  AST Builder (Rust)                              │
│  • Convert parse tree to AST                                     │
│  • Resolve imports                                               │
│  • Build symbol table                                            │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│              Type Checker (Rust)                                 │
│  • Type inference (Hindley-Milner)                              │
│  • Constraint generation & solving                               │
│  • Generic instantiation                                         │
│  • Error reporting                                               │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Typed AST                                     │
└──────┬────────────┬───────────┬────────────┬────────────────────┘
       │            │           │            │
       ▼            ▼           ▼            ▼
┌──────────┐  ┌─────────┐  ┌────────┐  ┌─────────┐
│  .NET    │  │   JVM   │  │ Native │  │  WASM   │
│  IL/CIL  │  │Bytecode │  │  LLVM  │  │         │
└────┬─────┘  └────┬────┘  └───┬────┘  └────┬────┘
     │             │            │            │
     ▼             ▼            ▼            ▼
┌─────────┐  ┌──────────┐  ┌────────┐  ┌──────────┐
│ .NET    │  │  JVM/    │  │ Native │  │ Browser/ │
│ Runtime │  │ Android  │  │  Exe   │  │  Deno    │
└─────────┘  └──────────┘  └────────┘  └──────────┘
```

## Module Breakdown

### 1. Rust Core (`src/`)

#### Parser Module (`src/parser/`)
- **Lexer** - Custom indentation-aware lexer
- **ANTLR Integration** - Generated parser from grammar
- **AST Conversion** - Parse tree to AST transformation

#### AST Module (`src/ast/`)
- **Type Definitions** - All AST node types
- **Visitors** - AST traversal patterns
- **Transformers** - AST optimization passes

#### Type Checker (`src/typechecker/`)
- **Symbol Table** - Scope and binding management
- **Type Inference** - Hindley-Milner algorithm
- **Constraint Solver** - Unification algorithm
- **Error Reporter** - Detailed type error messages

#### Code Generation (`src/codegen/`)
- **Backend Trait** - Common interface for all backends
- **.NET Backend** - CIL/IL generation (via `netcorehost`)
- **JVM Backend** - JVM bytecode (via FFI to Kotlin)
- **LLVM Backend** - Native code generation (via `inkwell`)

#### Runtime (`src/runtime/`)
- **FFI Exports** - C-compatible interface for other languages
- **Lua Integration** - Macro and build script support
- **Python Integration** - Tooling and analysis
- **JS Integration** - Web tooling

#### LSP Server (`src/lsp/`)
- **Protocol Handler** - LSP message routing
- **Diagnostics** - Real-time error reporting
- **Completion** - IntelliSense support
- **Navigation** - Go-to-definition, find references
- **Formatting** - Code formatting

### 2. Kotlin/JVM Components (`kotlin/`)

#### Compiler (`kotlin/compiler/`)
```
org.boolang.compiler
├── JvmCodegen.kt          # JVM bytecode generation
├── AndroidCodegen.kt      # Android-specific codegen
├── BoolangNative.kt       # JNA interface to Rust
└── CodegenBackend.kt      # Backend interface
```

**Key Classes:**
- `JvmCodegen` - Generates `.class` files using ASM
- `AndroidCodegen` - Generates Android-compatible bytecode
- `BoolangNative` - JNA bindings to Rust compiler

#### Runtime (`kotlin/runtime/`)
```
org.boolang.runtime
├── BooRuntime.kt          # Core runtime support
├── Collections.kt         # Collection implementations
├── Reflection.kt          # Runtime reflection
└── Interop.kt            # Java interop helpers
```

#### Standard Library (`kotlin/stdlib/`)
```
org.boolang.stdlib
├── Collections.kt         # List, Map, Set
├── IO.kt                 # File I/O
├── Strings.kt            # String utilities
└── Math.kt               # Math functions
```

### 3. Android Module (`android/`)

```
org.boolang.android
├── BoolangActivity        # Base Activity class
├── BoolangApplication     # Application class
├── Runtime                # Android-specific runtime
└── NativeLib             # JNI wrapper for Rust
```

**Features:**
- Native library loading
- Android lifecycle integration
- Permission handling
- Resource access

### 4. Dynamic Runtime Scripts (`scripts/`)

#### Lua (`scripts/lua/`)
- **Macros** - Code generation macros
- **Build Scripts** - Custom build logic
- **DSL Extensions** - Domain-specific languages

#### Python (`scripts/python/`)
- **Code Analysis** - Complexity metrics, linting
- **Tooling** - Code formatters, refactoring tools
- **Testing** - Test generators

#### JavaScript (`scripts/js/`)
- **Web Integration** - Browser compatibility
- **Node.js Tools** - CLI utilities
- **Build Tools** - Webpack/Vite plugins

#### Kotlin (`scripts/kotlin/`)
- **Gradle Scripts** - Build automation
- **Android Tools** - APK generation helpers
- **JVM Utilities** - Bytecode inspection

## Data Flow

### Compilation Pipeline

```
1. Source File (hello.boo)
   ↓
2. Lexer → Tokens
   ↓
3. Parser (ANTLR4) → Parse Tree
   ↓
4. AST Builder → AST
   ↓
5. Type Checker → Typed AST + Errors
   ↓
6. Backend Selection (.NET/JVM/Native/WASM)
   ↓
7. Code Generation → Binary/Bytecode
   ↓
8. Output File (hello.dll / hello.class / hello.exe / hello.wasm)
```

### LSP Flow

```
IDE/Editor ←→ LSP Protocol ←→ BooLang LSP Server
                                    ↓
                              Parser + Type Checker
                                    ↓
                              Diagnostics/Completion/etc
```

### Android Build Flow

```
.boo Source Files
   ↓
Rust Compiler (via FFI)
   ↓
Kotlin Codegen → JVM Bytecode
   ↓
Android Gradle Plugin
   ↓
DEX Bytecode
   ↓
APK Packaging
   ↓
Signed APK
```

## FFI Boundaries

### Rust ↔ Kotlin (JNA)

```rust
// Rust side
#[no_mangle]
pub extern "C" fn boolang_parse(source: *const c_char) -> *mut c_void
```

```kotlin
// Kotlin side
interface BoolangNative : Library {
    fun boolang_parse(source: String): Long
}
```

### Rust ↔ Lua (mlua)

```rust
// Rust side
lua.context(|ctx| {
    let globals = ctx.globals();
    globals.set("compile", ctx.create_function(|_, code: String| {
        // Compile Boo code
    })?)?;
})
```

```lua
-- Lua side
local ast = compile("class Foo:\n    pass")
```

### Rust ↔ Python (PyO3)

```rust
// Rust side
#[pyfunction]
fn parse_boo(source: &str) -> PyResult<Ast>
```

```python
# Python side
from boolang import parse_boo
ast = parse_boo("class Foo:\n    pass")
```

## Build System Integration

### Cargo (Rust)

```toml
[features]
default = ["dotnet", "jvm", "lua", "python", "javascript"]
dotnet = ["netcorehost"]
jvm = ["jni", "j4rs"]
# ... etc
```

### Gradle (Kotlin/Android)

```kotlin
tasks.register("buildRustCore") {
    // Build Rust before Kotlin
}

tasks.compileKotlin.dependsOn("buildRustCore")
```

## Threading Model

### Rust Core
- **Main Thread** - Compilation pipeline
- **Worker Threads** - Parallel module compilation (via `rayon`)

### LSP Server
- **Async Runtime** - `tokio` for non-blocking I/O
- **Request Handler** - Concurrent request processing

### Dynamic Runtimes
- **Lua** - Single-threaded (per-context)
- **Python** - GIL-aware threading
- **JavaScript** - Event loop integration

## Memory Management

### Rust
- **Stack** - Local variables, small structs
- **Heap** - AST nodes, large data structures
- **Arc/Rc** - Shared ownership where needed

### Kotlin/JVM
- **GC Managed** - All objects
- **Native References** - Careful management of Rust pointers

### FFI
- **Explicit Free** - `boolang_free()` for Rust allocations
- **Owned Pointers** - Clear ownership across FFI boundary

## Error Handling

### Rust
```rust
Result<T, Error> // Explicit error handling
anyhow::Error    // Flexible error types
```

### Kotlin
```kotlin
sealed class CompilerResult<T> {
    data class Success<T>(val value: T) : CompilerResult<T>()
    data class Failure<T>(val error: CompilerError) : CompilerResult<T>()
}
```

### LSP
- JSON-RPC error responses
- Diagnostic messages for IDE

## Performance Considerations

### Hot Paths
1. **Lexing** - Optimized for large files
2. **Type Checking** - Caching and incremental
3. **Codegen** - Parallel module compilation

### Caching
- **AST Cache** - Parsed files
- **Type Cache** - Inferred types
- **Incremental Compilation** - Only recompile changed modules

### Optimization Levels
- `-O0` - No optimization (fast compile)
- `-O1` - Basic optimization
- `-O2` - Full optimization
- `-O3` - Aggressive optimization

## Security

### Sandboxing
- **Lua Scripts** - Restricted API access
- **Python Scripts** - No `eval()` by default

### Code Signing
- **Android APK** - Signed with keystore
- **Windows Exe** - Authenticode signing (optional)

## Extensibility Points

1. **Custom Backends** - Implement `CodegenBackend` trait
2. **Macro System** - Lua/Python for code generation
3. **LSP Extensions** - Custom commands
4. **Build Plugins** - Gradle plugin API

## Future Enhancements

- [ ] Incremental compilation
- [ ] Debugger support (DAP)
- [ ] Package manager
- [ ] REPL
- [ ] Source maps for debugging
- [ ] Hot reload for Android
- [ ] WebAssembly System Interface (WASI)
- [ ] Native GUI support

## Related Documentation

- [README.md](README.md) - Project overview
- [GETTING_STARTED.md](GETTING_STARTED.md) - Setup guide
- [grammar/BooModern.g4](grammar/BooModern.g4) - Language syntax
