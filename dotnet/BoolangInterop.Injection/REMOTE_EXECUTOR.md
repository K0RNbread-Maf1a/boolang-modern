# Remote Executor via DLL Injection

## ⚠️ LEGAL DISCLAIMER

**THIS TOOL IS PROVIDED FOR AUTHORIZED TESTING AND EDUCATIONAL PURPOSES ONLY**

### Authorized Use Cases
✅ Debugging your own applications  
✅ Security testing your own software  
✅ Educational purposes on isolated test systems  
✅ Authorized penetration testing with written permission  
✅ Reverse engineering your own binaries  

### Prohibited Use
❌ Injecting into processes you don't own  
❌ Unauthorized access to computer systems  
❌ Bypassing security controls without permission  
❌ Any malicious or illegal activity  

**Violation of these terms may result in criminal prosecution under computer fraud laws.**

---

## Overview

The Remote Executor allows you to execute code in a remote process via DLL injection. This is useful for:

- **Debugging**: Inject diagnostic code into running processes
- **Testing**: Verify security controls and detection mechanisms
- **Research**: Study process behavior and inter-process communication
- **Development**: Test plugin architectures and extensions

## Architecture

```
┌─────────────────┐
│  Your C# App    │
│  (Executor)     │
└────────┬────────┘
         │ 1. Find Process
         │ 2. Inject DLL
         ▼
┌─────────────────┐      ┌──────────────┐
│ Target Process  │◄────►│  Injected    │
│                 │ RPC  │  Boolang DLL │
└─────────────────┘      └──────────────┘
                            │
                            │ 3. Execute Script
                            │ 4. Return Results
                            ▼
```

## Components

### 1. RemoteExecutor.cs
Main class for remote execution:
- Finds target process by name
- Injects Boolang DLL
- Sends scripts via RPC
- Receives execution results

### 2. InjectionHelper.cs
Low-level DLL injection (from previous work):
- OpenProcess
- VirtualAllocEx
- WriteProcessMemory
- CreateRemoteThread

### 3. InjectionRpc.cs
Communication with injected DLL:
- Named pipes for bidirectional communication
- Command serialization
- Result deserialization

## Usage

### Basic Example

```csharp
using BoolangInterop.Injection;

// Create executor
var executor = new RemoteExecutor(
    targetProcessName: "notepad",
    boolangDllPath: @"C:\path\to\boolang_modern.dll"
);

// Execute script
var result = executor.ExecuteScript(@"
    print('Hello from notepad process!')
    import os
    print(f'PID: {os.getpid()}')
");

Console.WriteLine(result);
```

### Command Line Tool

```powershell
# Build the tool
dotnet build

# Run with parameters
.\RemoteExecutor.exe notepad C:\path\to\boolang.dll
```

## Security Considerations

### 1. Permissions Required
- **Administrator/Root**: Required for OpenProcess with PROCESS_ALL_ACCESS
- **SeDebugPrivilege**: May be required for system processes

### 2. Detection
This technique is well-known and detected by:
- Antivirus software
- EDR (Endpoint Detection & Response)
- Windows Defender
- Process monitors (Process Explorer, Process Hacker)

### 3. Stability
- Improper injection can crash target process
- Incompatible DLLs can cause instability
- Memory leaks if cleanup is not proper

### 4. Target Process Selection
**Safe targets for testing:**
- Notepad (notepad.exe)
- Calculator (calc.exe)
- Your own test applications

**DO NOT target:**
- System processes (csrss.exe, winlogon.exe)
- Security software (antivirus, firewall)
- Other users' processes
- Production systems

## Implementation Details

### DLL Injection Flow

1. **Find Process**: Enumerate running processes by name
2. **Open Handle**: OpenProcess with required permissions
3. **Allocate Memory**: VirtualAllocEx in target process
4. **Write DLL Path**: WriteProcessMemory with DLL path string
5. **Create Thread**: CreateRemoteThread pointing to LoadLibraryA
6. **Wait**: Thread loads DLL, DllMain executes
7. **RPC Setup**: Injected DLL creates named pipe listener
8. **Communication**: Send commands, receive results

### Communication Protocol

```
Client (Executor)          Server (Injected DLL)
     │                            │
     ├─────── Connect ────────────►
     │                            │
     ├─────── Command ────────────►
     │        (JSON)              │
     │                            ├─ Execute
     │                            │
     │◄──────── Result ───────────┤
     │         (JSON)             │
     │                            │
     ├─────── Disconnect ─────────►
```

### Message Format

```json
{
  "type": "execute",
  "script": "print('Hello')",
  "timeout": 5000
}
```

Response:
```json
{
  "success": true,
  "output": "Hello\n",
  "error": null,
  "duration": 123
}
```

## Testing

### Safe Test Scenario

```csharp
// Start a test process you own
var process = Process.Start("notepad.exe");
Thread.Sleep(1000);

// Inject and execute
var executor = new RemoteExecutor("notepad", @"C:\path\to\boolang.dll");
var result = executor.ExecuteScript("print('Test successful')");

// Verify
Assert.IsTrue(result.Success);
Assert.Contains("Test successful", result.Output);

// Cleanup
process.Kill();
```

### Unit Tests

```csharp
[TestMethod]
public void TestRemoteExecution()
{
    // Arrange
    var testProcess = StartTestProcess();
    var executor = new RemoteExecutor("test-app", GetDllPath());
    
    // Act
    var result = executor.ExecuteScript("return 42");
    
    // Assert
    Assert.IsTrue(result.Success);
    Assert.AreEqual("42", result.Output.Trim());
    
    // Cleanup
    testProcess.Kill();
}
```

## Troubleshooting

### "Access Denied" Error
- Run as Administrator
- Check antivirus hasn't blocked injection
- Verify target process architecture matches (x64/x86)

### "DLL not found" Error
- Verify DLL path is correct and absolute
- Check DLL exists and is accessible
- Ensure DLL is compiled for target architecture

### Target Process Crashes
- DLL may have unhandled exceptions
- Incompatible DLL (architecture mismatch)
- Thread initialization issues
- Check DllMain implementation

### No Response from Injected DLL
- RPC pipe not created
- Firewall blocking named pipes
- DLL failed to initialize
- Check logs in injected DLL

## Building the Payload DLL

The injected DLL needs:

```c
// DllMain - entry point
BOOL APIENTRY DllMain(HMODULE hModule, DWORD reason, LPVOID lpReserved)
{
    if (reason == DLL_PROCESS_ATTACH)
    {
        // Initialize Boolang runtime
        boolang_init();
        
        // Create RPC listener thread
        CreateThread(NULL, 0, RpcListenerThread, NULL, 0, NULL);
    }
    return TRUE;
}

// RPC listener
DWORD WINAPI RpcListenerThread(LPVOID lpParam)
{
    // Create named pipe
    // Listen for commands
    // Execute scripts
    // Send results back
}
```

## Alternatives to Injection

If DLL injection is too invasive, consider:

1. **Process Hollowing**: Replace process memory
2. **AppInit_DLLs**: Registry-based loading (deprecated)
3. **COM Hijacking**: Hijack COM objects
4. **Debugging API**: Use debug APIs legally
5. **Named Pipes**: Direct RPC without injection

## References

- [Windows Process Injection Techniques](https://attack.mitre.org/techniques/T1055/)
- [MSDN: DLL Injection](https://docs.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-security)
- [Detecting DLL Injection](https://www.elastic.co/blog/detecting-dll-injection)

## License

MIT OR Apache-2.0

**Use responsibly and legally.**
