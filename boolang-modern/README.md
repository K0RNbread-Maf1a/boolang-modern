# BooLang Modern

A modern, multi-platform implementation of the Boo programming language with LSP support.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Core Compiler (Rust)                      │
│  ┌──────────┐  ┌──────────┐  ┌────────────┐  ┌─────────┐  │
│  │  Parser  │→│   AST    │→│ Type Check │→│ Codegen │  │
│  │ (ANTLR4) │  │          │  │  System    │  │         │  │
│  └──────────┘  └──────────┘  └────────────┘  └─────────┘  │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              LSP Server (tower-lsp)                   │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            ↓
    ┌───────────────────────┴────────────────────────┐
    │                                                 │
┌───▼───────┐    ┌──────────────┐    ┌──────────────▼───┐
│  .NET/CLR │    │   JVM/Android │    │  Native/WASM     │
│  Backend  │    │    Backend    │    │   Backend        │
└───────────┘    └───────────────┘    └──────────────────┘

┌─────────────────────────────────────────────────────────────┐
│              Dynamic Runtime Integrations                    │
│  ┌──────┐  ┌────────┐  ┌────────────┐  ┌──────────────┐   │
│  │ Lua  │  │ Python │  │ JavaScript │  │   Kotlin     │   │
│  │Macros│  │Analysis│  │  Tooling   │  │JVM/Android  │   │
│  └──────┘  └────────┘  └────────────┘  └──────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Target Platforms

### Runtime Targets
- **.NET** (CLR/CoreCLR) - Windows, Linux, macOS
- **JVM** - Java Virtual Machine
- **Android** - Native Android APKs
- **Native** - via LLVM
- **WebAssembly** - Browser and Deno

### Dynamic Runtime Support
- **Lua 5.4** - Build scripts, macros, simple extensions
- **Python 3.x** - Tooling, code analysis, complex transformations
- **JavaScript/TypeScript** - IDE plugins, web tooling
- **Kotlin** - JVM bytecode generation, Android integration

## Features

- 🎯 Python-like indentation-based syntax
- 🔒 Static typing with type inference
- 🚀 High performance via Rust core
- 🔧 LSP server for modern IDE support
- 📱 First-class Android development
- 🌐 Multi-platform targeting
- 🔌 Extensible via dynamic runtimes
- 🎨 Meta-programming capabilities

## Project Structure

```
boolang-modern/
├── src/                    # Rust core compiler
│   ├── parser/            # ANTLR4 parser integration
│   ├── ast/               # Abstract Syntax Tree
│   ├── typechecker/       # Type inference system
│   ├── codegen/           # Multi-backend code generation
│   ├── lsp/               # Language Server Protocol
│   └── runtime/           # Runtime support & FFI
├── grammar/               # ANTLR4 grammar files
│   └── BooModern.g4      # Language grammar
├── kotlin/                # Kotlin/JVM backend
│   ├── compiler/         # JVM bytecode generation
│   ├── runtime/          # Kotlin runtime support
│   └── stdlib/           # Standard library (Kotlin)
├── android/              # Android library module
├── gradle-plugins/       # Gradle plugins for builds
├── scripts/              # Dynamic runtime scripts
│   ├── lua/             # Lua build scripts
│   ├── python/          # Python tooling
│   ├── js/              # JavaScript utilities
│   └── kotlin/          # Kotlin scripts
├── vscode-extension/     # VS Code extension
├── examples/             # Example Boo programs
└── tests/                # Test suite
```

## Prerequisites

### Required
- **Rust** 1.75+ (`rustup install stable`)
- **Cargo** (comes with Rust)
- **ANTLR4** runtime

### Optional (based on target)
- **Java 17+** (for JVM/Android targets)
- **Gradle 8.5+** (for Kotlin/Android builds)
- **Android SDK** (for Android APK builds)
- **LLVM 15+** (for native compilation)
- **.NET SDK 8+** (for .NET targeting)
- **Python 3.10+** (for Python tooling)
- **Node.js 20+** (for JS tooling)
- **Lua 5.4** (for Lua scripts)

## Quick Start

### 1. Clone and Build

```powershell
# Clone the repository
git clone <repo-url> boolang-modern
cd boolang-modern

# Build Rust core
cargo build --release

# Build Kotlin/JVM components (optional)
gradle build

# Build Android library (optional)
gradle :android:assembleRelease
```

### 2. Install LSP Server

```powershell
# Install the LSP server binary
cargo install --path . --bin boolang-lsp

# Or use it directly
./target/release/boolang-lsp.exe
```

### 3. Install VS Code Extension

```powershell
cd vscode-extension
npm install
npm run compile
code --install-extension boolang-modern-*.vsix
```

### 4. Hello World

Create `hello.boo`:

```boo
namespace HelloWorld

import System

class Program:
    static def Main():
        print "Hello, BooLang Modern!"
```

Compile and run:

```powershell
# Compile to .NET
boolang compile --target dotnet hello.boo

# Compile to JVM
boolang compile --target jvm hello.boo

# Compile to Android APK
boolang android-build hello.boo
```

## Building for Different Targets

### .NET/CoreCLR

```powershell
cargo build --features dotnet
boolang compile --target dotnet myapp.boo
dotnet myapp.dll
```

### JVM/Kotlin

```powershell
gradle :kotlin:compiler:build
boolang compile --target jvm myapp.boo
java -jar myapp.jar
```

### Android APK

```powershell
# Set up Android environment
export ANDROID_HOME=/path/to/android-sdk

# Build Android library
gradle :android:assembleRelease

# Compile Boo to Android
boolang android-package com.example.myapp myapp.boo

# Outputs: myapp.apk
```

### Native (LLVM)

```powershell
cargo build --features llvm
boolang compile --target native myapp.boo
./myapp
```

### WebAssembly

```powershell
boolang compile --target wasm myapp.boo
# Outputs: myapp.wasm
```

## Language Server Protocol (LSP)

The LSP server provides:
- ✅ Syntax highlighting
- ✅ Auto-completion
- ✅ Go to definition
- ✅ Find references
- ✅ Hover documentation
- ✅ Diagnostics (errors/warnings)
- ✅ Code formatting
- ✅ Refactoring support

### VS Code Setup

The extension is configured automatically. For manual setup:

```json
{
  "boolang.server.path": "boolang-lsp",
  "boolang.trace.server": "verbose"
}
```

## Dynamic Runtime Extensions

### Lua Macros

```lua
-- scripts/lua/macros.lua
function generate_properties(class_name, fields)
    local code = "class " .. class_name .. ":\n"
    for _, field in ipairs(fields) do
        code = code .. "    " .. field .. ": auto\n"
    end
    return code
end
```

### Python Code Analysis

```python
# scripts/python/analyzer.py
from boolang import ast

def analyze_complexity(source_file):
    tree = ast.parse_file(source_file)
    return calculate_cyclomatic_complexity(tree)
```

### Kotlin Android Integration

```kotlin
// kotlin/compiler/src/AndroidCodegen.kt
class AndroidCodegen : CodegenBackend {
    override fun generateActivity(cls: ClassDeclaration): ByteArray {
        return generateAndroidActivity(cls)
    }
}
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## Roadmap

- [x] Core parser and AST
- [x] Type inference system
- [x] LSP server foundation
- [ ] .NET IL code generation
- [ ] JVM bytecode generation
- [ ] Android tooling
- [ ] Standard library
- [ ] Package manager
- [ ] REPL
- [ ] Debugger protocol (DAP)

## License

MIT OR Apache-2.0

## Credits

Inspired by the original [Boo programming language](https://github.com/boo-lang/boo) by Rodrigo B. De Oliveira.
