//! FFI (Foreign Function Interface) for C interop
//!
//! This module provides C-compatible exports for use with DLL injection
//! and .NET interop via BoolangInterop

use crate::error::Result;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

/// Initialize the Boolang runtime
/// Returns 0 on success, non-zero on error
#[no_mangle]
pub extern "C" fn boolang_init() -> c_int {
    match crate::init() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Shutdown the Boolang runtime
#[no_mangle]
pub extern "C" fn boolang_shutdown() {
    // Cleanup any global state
}

/// Allocate memory
#[no_mangle]
pub extern "C" fn boolang_alloc(size: usize) -> *mut u8 {
    let mut vec = Vec::with_capacity(size);
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec);
    ptr
}

/// Free memory allocated by boolang_alloc
#[no_mangle]
pub extern "C" fn boolang_free(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

/// Create a Rust string from C string
#[no_mangle]
pub extern "C" fn boolang_string_new(str: *const c_char) -> *mut c_char {
    if str.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        let c_str = CStr::from_ptr(str);
        if let Ok(rust_str) = c_str.to_str() {
            if let Ok(owned) = CString::new(rust_str) {
                return owned.into_raw();
            }
        }
    }
    ptr::null_mut()
}

/// Get pointer to string data
#[no_mangle]
pub extern "C" fn boolang_string_ptr(rust_string: *mut c_char) -> *const c_char {
    rust_string as *const c_char
}

/// Free a Rust string
#[no_mangle]
pub extern "C" fn boolang_string_free(rust_string: *mut c_char) {
    if !rust_string.is_null() {
        unsafe {
            let _ = CString::from_raw(rust_string);
        }
    }
}

/// Parse Boolang source code
/// Returns AST handle on success, null on error
/// error_code is set to 0 on success, non-zero on error
#[no_mangle]
pub extern "C" fn boolang_parse(source_code: *mut c_char, error_code: *mut c_int) -> *mut u8 {
    if source_code.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            unsafe { *error_code = -1; }
        }
        return ptr::null_mut();
    }

    unsafe {
        let c_str = CStr::from_ptr(source_code);
        if let Ok(_source) = c_str.to_str() {
            // TODO: Implement actual parsing
            // For now, just return a dummy pointer
            *error_code = 0;
            boolang_alloc(64) // Placeholder
        } else {
            *error_code = -2;
            ptr::null_mut()
        }
    }
}

/// Compile AST to bytecode/IL
#[no_mangle]
pub extern "C" fn boolang_compile(ast: *mut u8, error_code: *mut c_int) -> *mut u8 {
    if ast.is_null() || error_code.is_null() {
        if !error_code.is_null() {
            unsafe { *error_code = -1; }
        }
        return ptr::null_mut();
    }

    unsafe {
        // TODO: Implement actual compilation
        *error_code = 0;
        boolang_alloc(128) // Placeholder
    }
}

/// Execute compiled code
/// Returns execution result code
#[no_mangle]
pub extern "C" fn boolang_execute(compiled_code: *mut u8) -> c_int {
    if compiled_code.is_null() {
        return -1;
    }

    // TODO: Implement actual execution
    0
}

/// Callback function type
pub type BoolangCallback = extern "C" fn(*const u8, c_int) -> c_int;

static mut CALLBACK: Option<BoolangCallback> = None;

/// Register a callback function
#[no_mangle]
pub extern "C" fn boolang_register_callback(callback: BoolangCallback) {
    unsafe {
        CALLBACK = Some(callback);
    }
}

/// Invoke the registered callback
pub fn invoke_callback(data: &[u8]) -> c_int {
    unsafe {
        if let Some(callback) = CALLBACK {
            callback(data.as_ptr(), data.len() as c_int)
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_shutdown() {
        assert_eq!(boolang_init(), 0);
        boolang_shutdown();
    }

    #[test]
    fn test_memory_allocation() {
        let ptr = boolang_alloc(100);
        assert!(!ptr.is_null());
        boolang_free(ptr);
    }

    #[test]
    fn test_string_operations() {
        let c_str = CString::new("test").unwrap();
        let rust_str = boolang_string_new(c_str.as_ptr());
        assert!(!rust_str.is_null());
        boolang_string_free(rust_str);
    }
}
