using System.Runtime.InteropServices;

namespace BoolangInterop.Core;

/// <summary>
/// Native interop wrapper for Rust FFI functions
/// </summary>
public static unsafe class RustFFI
{
    private const string DllName = "boolang_modern";

    #region Memory Management

    /// <summary>
    /// Allocates memory in Rust and returns a pointer
    /// </summary>
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr boolang_alloc(nuint size);

    /// <summary>
    /// Frees memory allocated by Rust
    /// </summary>
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void boolang_free(IntPtr ptr);

    #endregion

    #region String Handling

    /// <summary>
    /// Creates a Rust string from C# string
    /// </summary>
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
    public static extern IntPtr boolang_string_new([MarshalAs(UnmanagedType.LPStr)] string str);

    /// <summary>
    /// Gets C string from Rust string pointer
    /// </summary>
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr boolang_string_ptr(IntPtr rust_string);

    /// <summary>
    /// Frees a Rust string
    /// </summary>
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void boolang_string_free(IntPtr rust_string);

    #endregion

    #region Compiler/Parser Functions

    /// <summary>
    /// Initialize the Boolang runtime
    /// </summary>
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern int boolang_init();

    /// <summary>
    /// Shutdown the Boolang runtime
    /// </summary>
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void boolang_shutdown();

    /// <summary>
    /// Parse Boolang source code
    /// </summary>
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr boolang_parse(IntPtr source_code, out int error_code);

    /// <summary>
    /// Compile Boolang AST to bytecode/IL
    /// </summary>
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr boolang_compile(IntPtr ast, out int error_code);

    /// <summary>
    /// Execute compiled Boolang code
    /// </summary>
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern int boolang_execute(IntPtr compiled_code);

    #endregion

    #region Callback Support

    /// <summary>
    /// Function pointer type for C# callbacks from Rust
    /// </summary>
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate int BoolangCallback(IntPtr data, int data_len);

    /// <summary>
    /// Register a callback function that Rust can invoke
    /// </summary>
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void boolang_register_callback(BoolangCallback callback);

    #endregion
}
