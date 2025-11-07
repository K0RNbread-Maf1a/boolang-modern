# BoolangInterop - .NET to Rust FFI

This solution provides .NET interop for the Boolang language runtime written in Rust, enabling DLL injection and embedded scripting capabilities.

## Projects

### BoolangInterop.Core
Core FFI bindings for calling Rust functions from C#. Contains:
- `RustFFI.cs` - Low-level P/Invoke declarations
- `BoolangRuntime.cs` - High-level managed wrapper

### BoolangInterop.Injection
DLL injection utilities for Windows. Contains:
- `InjectionHelper.cs` - Classic DLL injection using CreateRemoteThread

## Building

```powershell
# Build the entire solution
dotnet build BoolangInterop.sln

# Build release version
dotnet build BoolangInterop.sln -c Release
```

## Rust Configuration

To enable FFI exports from your Rust project, update `Cargo.toml`:

```toml
[lib]
name = "boolang_modern"
crate-type = ["cdylib", "rlib"]
```

Then implement the FFI functions. See `rust_ffi_example.rs` for a complete example.

Build the Rust DLL:
```bash
cargo build --release
```

The DLL will be at `target/release/boolang_modern.dll` on Windows.

## Usage Examples

### Basic Runtime Usage

```csharp
using BoolangInterop.Core;

using var runtime = new BoolangRuntime();

string script = @"
def hello():
    print('Hello from Boolang!')
";

int result = runtime.ExecuteScript(script);
```

### DLL Injection

```csharp
using BoolangInterop.Injection;

int processId = InjectionHelper.FindProcessByName("target_process") ?? -1;
if (processId > 0)
{
    string dllPath = @"C:\path\to\boolang_modern.dll";
    bool success = InjectionHelper.InjectDll(processId, dllPath);
}
```

### Low-Level FFI

```csharp
using BoolangInterop.Core;

// Direct FFI calls
int result = RustFFI.boolang_init();
IntPtr sourcePtr = RustFFI.boolang_string_new("print('test')");
IntPtr ast = RustFFI.boolang_parse(sourcePtr, out int errorCode);

// Cleanup
RustFFI.boolang_string_free(sourcePtr);
RustFFI.boolang_free(ast);
RustFFI.boolang_shutdown();
```

## Required Rust FFI Functions

The following functions must be exported from your Rust library:

- `boolang_init()` - Initialize runtime
- `boolang_shutdown()` - Cleanup runtime
- `boolang_alloc(size)` - Allocate memory
- `boolang_free(ptr)` - Free memory
- `boolang_string_new(str)` - Create string
- `boolang_string_free(str)` - Free string
- `boolang_parse(source, error)` - Parse source code
- `boolang_compile(ast, error)` - Compile AST
- `boolang_execute(code)` - Execute compiled code
- `boolang_register_callback(fn)` - Register callback

## Platform Support

Currently supports:
- Windows x64 (DLL injection requires Windows)
- .NET 10.0

## Security Considerations

⚠️ **DLL Injection is a powerful technique that can be dangerous:**
- Only inject into processes you own or have permission to modify
- DLL injection may trigger antivirus/EDR solutions
- Use for legitimate purposes only (debugging, game modding, research)
- Requires administrative privileges on most systems

## Next Steps

1. Implement the Rust FFI functions in your main Rust project
2. Build the Rust cdylib
3. Test the interop with simple scripts
4. Implement script payload injection
5. Add support for marshaling complex data types
6. Consider adding reflective DLL injection for stealth
