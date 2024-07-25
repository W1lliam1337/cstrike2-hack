use std::num::ParseIntError;

use crate::common;
use common::*;

use windows::Win32::{
    Foundation::HMODULE,
    System::{
        LibraryLoader::{GetModuleHandleW, GetProcAddress},
        ProcessStatus::{GetModuleInformation, MODULEINFO},
        Threading::GetCurrentProcess,
    },
};

use windows::core::{PCSTR, PCWSTR};

/// Obtains a module handle by its name.
///
/// This function uses the `GetModuleHandleW` function from the Windows API to retrieve a handle to a
/// module by its name. The module name is specified as a UTF-8 encoded string, which is then converted
/// to UTF-16 for use with the Windows API.
///
/// # Parameters
///
/// * `module_name`: A string representing the name of the module to retrieve. The name should be the
///   base name of the module, without any path information.
///
/// # Return Value
///
/// Returns a raw pointer to the module handle if the module is found. If the module is not found,
/// the function returns `null`. The returned handle can be used with other Windows API functions to
/// interact with the module.
#[inline]
#[must_use]
pub fn get_module_handle(module_name: &str) -> Option<HMODULE> {
    // Convert the module name to a UTF-16 string with a null terminator
    let module_name_wide: Vec<u16> = module_name.encode_utf16().chain(std::iter::once(0)).collect();

    // SAFETY: The `module_name_wide` vector is valid and null-terminated. The call to `GetModuleHandleW`
    // expects a valid UTF-16 string and will return a handle to the module if it is loaded.
    // The result of `GetModuleHandleW` is converted to an `Option` using `.ok()`, handling `null` properly.
    unsafe { GetModuleHandleW(PCWSTR(module_name_wide.as_ptr())).ok() }
}

/// Retrieves the address of an exported function or variable from the specified module.
///
/// # Parameters
///
/// * `module_handle`: A handle to the module containing the function or variable.
/// * `proc_name`: The name of the function or variable to retrieve.
///
/// # Returns
///
/// * `Option<*mut c_void>`: The address of the function or variable if found, otherwise `None`.
///
/// # Panics
///
/// This function may panic if:
///
/// * `proc_name` contains a null byte or other invalid characters for a C string, which would cause
///   `CString::new` to panic.
///
/// # Note
///
/// The return value should be checked before use to avoid dereferencing null pointers.
#[inline]
#[must_use]
pub fn get_proc_address(module_handle: HMODULE, proc_name: &str) -> Option<*mut c_void> {
    let proc_name_cstr = match CString::new(proc_name) {
        Ok(cstr) => cstr,
        Err(_) => return None, // Return None if proc_name is invalid
    };

    // SAFETY: The caller must ensure that `module_handle` is a valid module handle and
    // `proc_name` is a valid function name. The external `GetProcAddress` function is
    // used to retrieve the function address. The result is cast to `*mut c_void`.
    unsafe {
        GetProcAddress(module_handle, PCSTR(proc_name_cstr.as_ptr().cast::<u8>()))
            .map(|addr| addr as *mut _)
    }
}

/// Retrieves module information for a given module handle.
///
/// This function uses the `GetModuleInformation` function from the Windows API to obtain detailed
/// information about a module, such as the base address, size of the image, and entry point.
///
/// # Parameters
///
/// * `module_handle`: A raw pointer to the module's handle. This can be obtained using the
///   `get_module_handle` function.
///
/// # Return Value
///
/// Returns `Some(module_info)` if the module information is successfully obtained.
/// The `module_info` is a `MODULEINFO` struct containing the base address, size of the image,
/// and entry point of the module.
///
/// Returns `None` if the module information cannot be obtained or if an error occurs.
#[inline]
#[must_use]
pub fn get_module_info(module_handle: HMODULE) -> Option<MODULEINFO> {
    let mut module_info =
        MODULEINFO { lpBaseOfDll: null_mut(), SizeOfImage: 0, EntryPoint: null_mut() };

    let size_of_module_info = match u32::try_from(size_of::<MODULEINFO>()) {
        Ok(size) => size,
        Err(_) => return None, // Return None if size conversion fails
    };

    // SAFETY: The caller must ensure that `module_handle` is a valid handle to a loaded module.
    unsafe {
        GetModuleInformation(
            GetCurrentProcess(),
            module_handle,
            &mut module_info,
            size_of_module_info,
        )
        .is_ok()
        .then_some(module_info) // Use `then_some` to simplify the return logic
    }
}

/// Searches for a pattern within the memory of a specified module.
///
/// This function uses a simple byte-by-byte comparison to find a pattern within the memory of a module.
/// The pattern is specified as a space-separated sequence of hexadecimal bytes, with "??" representing
/// a wildcard that matches any byte.
///
/// # Parameters
///
/// * `module_handle`: A handle to the module within which to search for the pattern.
///   This can be obtained using the `get_module_handle` function.
///
/// * `pattern`: A string representing the pattern to search for. The pattern should be a space-separated
///   sequence of hexadecimal bytes, with "??" representing a wildcard.
///
/// # Return Value
///
/// Returns `Some(address_offset)` if the pattern is found within the module's memory.
/// The `address_offset` is the memory address of the first byte of the pattern, relative to the base
/// address of the module.
///
/// Returns `None` if the pattern is not found within the module's memory.
///
/// # Panics
///
/// This function may panic if:
///
/// * The `pattern` string contains invalid hexadecimal characters.
/// * The `pattern` string contains a null byte or other invalid characters for a C string.
/// * The `address_offset` calculation overflows.
#[inline]
#[must_use]
pub fn pattern_search(module_handle: HMODULE, pattern: &str) -> Option<usize> {
    // Parse the pattern string into bytes and handle wildcards
    let parsed_pattern_bytes: Result<Vec<Option<u8>>, ParseIntError> =
        pattern
            .split_whitespace()
            .map(|byte_str| {
                if byte_str == "??" {
                    Ok(None)
                } else {
                    u8::from_str_radix(byte_str, 16).map(Some)
                }
            })
            .collect();

    // Handle parsing errors and continue if successful
    let pattern_bytes = match parsed_pattern_bytes {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to parse pattern: {e}");
            return None;
        }
    };

    // Retrieve module information
    let module_info = match get_module_info(module_handle) {
        Some(info) => info,
        None => return None,
    };

    let base_address = module_info.lpBaseOfDll;
    let size = match usize::try_from(module_info.SizeOfImage) {
        Ok(size) => size,
        Err(_) => return None,
    };

    // SAFETY: Convert base_address to a raw pointer for memory access
    let module_memory = unsafe {
        // Ensure the pointer and size are valid before creating a slice
        slice::from_raw_parts(base_address as *const u8, size)
    };

    for i in 0..module_memory.len().saturating_sub(pattern_bytes.len()) {
        if pattern_bytes
            .iter()
            .enumerate()
            .all(|(j, &b)| b.map_or(true, |b| module_memory[i + j] == b))
        {
            let address_offset = (base_address as usize)
                .checked_add(i)
                .ok_or_else(|| {
                    eprintln!("Address calculation overflowed");
                    None::<usize>
                })
                .expect("Failed to calculate address");

            return Some(address_offset);
        }
    }

    None
}

/// Retrieves a pointer to a specific interface from a module.
///
/// This function uses the `CreateInterface` function from the specified module to obtain a pointer to
/// a requested interface. The interface is identified by its name, which is passed as a parameter to
/// the function.
///
/// # Parameters
///
/// * `module_handle`: A handle to the module containing the `CreateInterface` function.
///   This can be obtained using the `get_module_handle` function.
///
/// * `interface_name`: A string representing the name of the interface to retrieve.
///   The name should match the name used by the module to identify the interface.
///
/// # Returns
///
/// * `Some(interface_ptr)`: If the interface is successfully retrieved. The `interface_ptr` is a raw pointer
///   to the requested interface.
///
/// * `None`: If the interface cannot be retrieved or if an error occurs.
///
/// # Panics
///
/// This function may panic if:
///
/// * `get_proc_address` returns `None`, which will cause `expect` to panic.
/// * `CString::new` fails to create a C-style string, which will also cause `expect` to panic.
///
/// # Note
///
/// The returned pointer is raw and should be used with caution. Ensure that the pointer is valid before
/// dereferencing or using it.
#[inline]
#[must_use]
pub fn get_interface(module_handle: HMODULE, interface_name: &str) -> Option<*const usize> {
    // SAFETY: We assume that `get_proc_address` returns a valid function pointer.
    let function: unsafe extern "C" fn(*const c_char, *const c_int) -> *const c_void = unsafe {
        get_proc_address(module_handle, "CreateInterface")
            .map(|addr| transmute(addr))
            .ok_or_else(|| {
                eprintln!("Failed to get function address for CreateInterface");
                None::<usize>
            })
            .expect("Failed to cast CreateInterface to a function pointer")
    };

    let interface_name_cstr = match CString::new(interface_name) {
        Ok(cstr) => cstr,
        Err(_) => {
            eprintln!("Failed to create CString from interface_name");
            return None;
        }
    };

    // SAFETY: We assume that `function` is a valid function pointer and `interface_name_cstr` is valid.
    Some(unsafe { function(interface_name_cstr.as_ptr(), null_mut()) as *const usize })
}
