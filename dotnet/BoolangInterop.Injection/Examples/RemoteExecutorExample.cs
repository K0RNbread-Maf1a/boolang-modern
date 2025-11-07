using BoolangInterop.Injection;

namespace BoolangInterop.Examples;

/// <summary>
/// Example usage of RemoteExecutor
/// ⚠️ FOR AUTHORIZED TESTING ONLY ⚠️
/// Only use on processes you own or have explicit permission to test
/// </summary>
public class RemoteExecutorExample
{
    public static void Main(string[] args)
    {
        Console.WriteLine("╔═══════════════════════════════════════════════════╗");
        Console.WriteLine("║     Boolang Remote Executor - Testing Tool       ║");
        Console.WriteLine("║     FOR AUTHORIZED USE ONLY                       ║");
        Console.WriteLine("╚═══════════════════════════════════════════════════╝\n");

        if (args.Length < 2)
        {
            PrintUsage();
            return;
        }

        string targetProcess = args[0];
        string dllPath = args[1];

        Console.WriteLine($"Target Process: {targetProcess}");
        Console.WriteLine($"DLL Path: {dllPath}\n");

        // Confirm with user
        Console.Write("⚠️  This will inject code into the target process. Continue? (yes/no): ");
        string? confirm = Console.ReadLine();
        if (confirm?.ToLower() != "yes")
        {
            Console.WriteLine("Aborted.");
            return;
        }

        try
        {
            var executor = new RemoteExecutor(targetProcess, dllPath);

            // Example 1: Simple execution
            Console.WriteLine("\n--- Example 1: Simple Script Execution ---");
            var result1 = executor.ExecuteScript(@"
                print('Hello from injected code!')
                return 42
            ");
            Console.WriteLine(result1);

            // Example 2: Process information gathering
            Console.WriteLine("\n--- Example 2: Process Information ---");
            var result2 = executor.ExecuteScript(@"
                import os
                import sys
                
                print('OS:', os.name)
                print('Python:', sys.version)
                print('PID:', os.getpid())
            ");
            Console.WriteLine(result2);

            // Example 3: Safe testing operation
            Console.WriteLine("\n--- Example 3: Safe Test ---");
            var result3 = executor.ExecuteScript(@"
                # Non-destructive test
                x = 1 + 1
                print(f'Test result: {x}')
            ");
            Console.WriteLine(result3);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"❌ Error: {ex.Message}");
        }
    }

    private static void PrintUsage()
    {
        Console.WriteLine("Usage: RemoteExecutor <target_process> <dll_path>");
        Console.WriteLine();
        Console.WriteLine("Examples:");
        Console.WriteLine("  RemoteExecutor notepad C:\\path\\to\\boolang.dll");
        Console.WriteLine("  RemoteExecutor calc C:\\path\\to\\boolang.dll");
        Console.WriteLine();
        Console.WriteLine("⚠️  WARNING:");
        Console.WriteLine("  - Only inject into processes you own");
        Console.WriteLine("  - Requires Administrator privileges");
        Console.WriteLine("  - May be flagged by antivirus software");
        Console.WriteLine("  - For testing and debugging purposes only");
        Console.WriteLine();
        Console.WriteLine("Legal Use Cases:");
        Console.WriteLine("  ✓ Debugging your own applications");
        Console.WriteLine("  ✓ Testing security of your own software");
        Console.WriteLine("  ✓ Educational purposes on isolated systems");
        Console.WriteLine("  ✓ Authorized penetration testing");
        Console.WriteLine();
        Console.WriteLine("  ✗ Injecting into other users' processes");
        Console.WriteLine("  ✗ Unauthorized access to systems");
        Console.WriteLine("  ✗ Malicious purposes");
    }
}
