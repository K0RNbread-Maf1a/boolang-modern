using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Text;

namespace BoolangInterop.Injection;

/// <summary>
/// Provides DLL injection capabilities for Windows processes
/// </summary>
public static unsafe class InjectionHelper
{
    #region Windows API

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern IntPtr OpenProcess(uint dwDesiredAccess, bool bInheritHandle, int dwProcessId);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern IntPtr VirtualAllocEx(IntPtr hProcess, IntPtr lpAddress, nuint dwSize, 
        uint flAllocationType, uint flProtect);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool VirtualFreeEx(IntPtr hProcess, IntPtr lpAddress, nuint dwSize, uint dwFreeType);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool WriteProcessMemory(IntPtr hProcess, IntPtr lpBaseAddress, byte[] lpBuffer, 
        nuint nSize, out nuint lpNumberOfBytesWritten);

    [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Ansi)]
    private static extern IntPtr GetProcAddress(IntPtr hModule, string lpProcName);

    [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Ansi)]
    private static extern IntPtr GetModuleHandle(string lpModuleName);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern IntPtr CreateRemoteThread(IntPtr hProcess, IntPtr lpThreadAttributes, nuint dwStackSize,
        IntPtr lpStartAddress, IntPtr lpParameter, uint dwCreationFlags, out IntPtr lpThreadId);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern uint WaitForSingleObject(IntPtr hHandle, uint dwMilliseconds);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern bool CloseHandle(IntPtr hObject);

    private const uint PROCESS_CREATE_THREAD = 0x0002;
    private const uint PROCESS_QUERY_INFORMATION = 0x0400;
    private const uint PROCESS_VM_OPERATION = 0x0008;
    private const uint PROCESS_VM_WRITE = 0x0020;
    private const uint PROCESS_VM_READ = 0x0010;

    private const uint MEM_COMMIT = 0x1000;
    private const uint MEM_RESERVE = 0x2000;
    private const uint MEM_RELEASE = 0x8000;
    private const uint PAGE_READWRITE = 0x04;

    private const uint INFINITE = 0xFFFFFFFF;

    #endregion

    /// <summary>
    /// Inject a DLL into a target process
    /// </summary>
    /// <param name="processId">Target process ID</param>
    /// <param name="dllPath">Full path to the DLL to inject</param>
    /// <returns>True if injection was successful</returns>
    public static bool InjectDll(int processId, string dllPath)
    {
        if (!File.Exists(dllPath))
            throw new FileNotFoundException("DLL not found", dllPath);

        IntPtr hProcess = IntPtr.Zero;
        IntPtr allocMemAddress = IntPtr.Zero;
        IntPtr hThread = IntPtr.Zero;

        try
        {
            // Open target process
            hProcess = OpenProcess(
                PROCESS_CREATE_THREAD | PROCESS_QUERY_INFORMATION | 
                PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_VM_READ,
                false, processId);

            if (hProcess == IntPtr.Zero)
            {
                throw new InjectionException($"Failed to open process {processId}. Error: {Marshal.GetLastWin32Error()}");
            }

            // Get LoadLibraryA address
            IntPtr kernel32 = GetModuleHandle("kernel32.dll");
            IntPtr loadLibraryAddr = GetProcAddress(kernel32, "LoadLibraryA");

            if (loadLibraryAddr == IntPtr.Zero)
            {
                throw new InjectionException("Failed to get LoadLibraryA address");
            }

            // Allocate memory in target process
            byte[] dllBytes = Encoding.ASCII.GetBytes(dllPath + "\0");
            allocMemAddress = VirtualAllocEx(hProcess, IntPtr.Zero, (nuint)dllBytes.Length,
                MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);

            if (allocMemAddress == IntPtr.Zero)
            {
                throw new InjectionException($"Failed to allocate memory in target process. Error: {Marshal.GetLastWin32Error()}");
            }

            // Write DLL path to target process memory
            if (!WriteProcessMemory(hProcess, allocMemAddress, dllBytes, (nuint)dllBytes.Length, out nuint bytesWritten))
            {
                throw new InjectionException($"Failed to write to target process memory. Error: {Marshal.GetLastWin32Error()}");
            }

            // Create remote thread to load DLL
            hThread = CreateRemoteThread(hProcess, IntPtr.Zero, 0, loadLibraryAddr, 
                allocMemAddress, 0, out IntPtr threadId);

            if (hThread == IntPtr.Zero)
            {
                throw new InjectionException($"Failed to create remote thread. Error: {Marshal.GetLastWin32Error()}");
            }

            // Wait for thread to complete
            WaitForSingleObject(hThread, INFINITE);

            return true;
        }
        finally
        {
            // Cleanup
            if (allocMemAddress != IntPtr.Zero && hProcess != IntPtr.Zero)
                VirtualFreeEx(hProcess, allocMemAddress, 0, MEM_RELEASE);
            
            if (hThread != IntPtr.Zero)
                CloseHandle(hThread);
            
            if (hProcess != IntPtr.Zero)
                CloseHandle(hProcess);
        }
    }

    /// <summary>
    /// Inject Boolang script as embedded payload into target process
    /// </summary>
    public static bool InjectScript(int processId, string scriptContent, string boolangDllPath)
    {
        // First inject the Boolang runtime DLL
        if (!InjectDll(processId, boolangDllPath))
            return false;

        // TODO: Implement script payload injection
        // This would require creating a stub DLL that:
        // 1. Loads the Boolang runtime
        // 2. Executes the provided script
        // 3. Can communicate results back

        throw new NotImplementedException("Script injection not yet implemented");
    }

    /// <summary>
    /// Find process by name
    /// </summary>
    public static int? FindProcessByName(string processName)
    {
        var processes = Process.GetProcessesByName(processName);
        return processes.Length > 0 ? processes[0].Id : null;
    }
}

/// <summary>
/// Exception thrown during DLL injection operations
/// </summary>
public class InjectionException : Exception
{
    public InjectionException(string message) : base(message) { }
    public InjectionException(string message, Exception inner) : base(message, inner) { }
}
