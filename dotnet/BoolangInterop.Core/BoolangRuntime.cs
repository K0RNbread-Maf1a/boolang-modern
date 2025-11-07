using System.Runtime.InteropServices;
using System.Text;

namespace BoolangInterop.Core;

/// <summary>
/// High-level managed wrapper for Boolang runtime
/// </summary>
public class BoolangRuntime : IDisposable
{
    private bool _initialized;
    private bool _disposed;

    public BoolangRuntime()
    {
        Initialize();
    }

    /// <summary>
    /// Initialize the Boolang runtime
    /// </summary>
    public void Initialize()
    {
        if (_initialized) return;

        var result = RustFFI.boolang_init();
        if (result != 0)
        {
            throw new BoolangException($"Failed to initialize Boolang runtime. Error code: {result}");
        }

        _initialized = true;
    }

    /// <summary>
    /// Parse Boolang source code and return AST handle
    /// </summary>
    public IntPtr Parse(string sourceCode)
    {
        ThrowIfNotInitialized();

        IntPtr sourcePtr = RustFFI.boolang_string_new(sourceCode);
        try
        {
            IntPtr ast = RustFFI.boolang_parse(sourcePtr, out int errorCode);
            if (errorCode != 0)
            {
                throw new BoolangException($"Parse failed with error code: {errorCode}");
            }
            return ast;
        }
        finally
        {
            RustFFI.boolang_string_free(sourcePtr);
        }
    }

    /// <summary>
    /// Compile AST to executable code
    /// </summary>
    public IntPtr Compile(IntPtr ast)
    {
        ThrowIfNotInitialized();

        IntPtr compiled = RustFFI.boolang_compile(ast, out int errorCode);
        if (errorCode != 0)
        {
            throw new BoolangException($"Compilation failed with error code: {errorCode}");
        }
        return compiled;
    }

    /// <summary>
    /// Execute compiled Boolang code
    /// </summary>
    public int Execute(IntPtr compiledCode)
    {
        ThrowIfNotInitialized();
        return RustFFI.boolang_execute(compiledCode);
    }

    /// <summary>
    /// Execute Boolang source code directly
    /// </summary>
    public int ExecuteScript(string sourceCode)
    {
        IntPtr ast = IntPtr.Zero;
        IntPtr compiled = IntPtr.Zero;

        try
        {
            ast = Parse(sourceCode);
            compiled = Compile(ast);
            return Execute(compiled);
        }
        finally
        {
            if (ast != IntPtr.Zero)
                RustFFI.boolang_free(ast);
            if (compiled != IntPtr.Zero)
                RustFFI.boolang_free(compiled);
        }
    }

    private void ThrowIfNotInitialized()
    {
        if (!_initialized)
            throw new InvalidOperationException("Boolang runtime is not initialized");
        if (_disposed)
            throw new ObjectDisposedException(nameof(BoolangRuntime));
    }

    public void Dispose()
    {
        if (_disposed) return;

        if (_initialized)
        {
            RustFFI.boolang_shutdown();
            _initialized = false;
        }

        _disposed = true;
        GC.SuppressFinalize(this);
    }

    ~BoolangRuntime()
    {
        Dispose();
    }
}

/// <summary>
/// Exception thrown by Boolang runtime operations
/// </summary>
public class BoolangException : Exception
{
    public BoolangException(string message) : base(message) { }
    public BoolangException(string message, Exception inner) : base(message, inner) { }
}
