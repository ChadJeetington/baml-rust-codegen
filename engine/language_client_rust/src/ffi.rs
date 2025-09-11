//! FFI bindings to the shared BAML runtime library
//!
//! This module re-exports the FFI functions and types from baml_cffi,
//! providing a clean interface for the Rust client.

// Re-export the FFI functions from baml_cffi
pub use baml_cffi::{
    register_callbacks, CallbackFn, OnTickCallbackFn,
    call_function_from_c, call_function_parse_from_c, call_function_stream_from_c,
    call_object_constructor, call_object_method, free_buffer, Buffer,
    create_baml_runtime, destroy_baml_runtime, invoke_runtime_cli, version,
};

// Re-export the protobuf types
pub use baml_cffi::baml;

/// Get the version of the BAML library
pub fn get_library_version() -> Result<String, String> {
    let version_ptr = version();
    if version_ptr.is_null() {
        return Err("Failed to get library version".to_string());
    }
    let version = unsafe {
        std::ffi::CStr::from_ptr(version_ptr)
            .to_str()
            .map_err(|e| format!("Invalid UTF-8 in version string: {}", e))?
    };
    Ok(version.to_string())
}
