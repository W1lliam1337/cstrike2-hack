use crate::common;
use common::*;

use std::{ffi::CString, slice};

use winapi::um::{
    libloaderapi::{GetModuleHandleW, GetProcAddress},
    processthreadsapi::GetCurrentProcess,
    psapi::{GetModuleInformation, MODULEINFO},
};

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
pub fn get_module_handle(module_name: &str) -> *mut c_void {
    let collect = module_name.encode_utf16().chain(std::iter::once(0)).collect();
    let module_name_wide: Vec<u16> = collect;
    unsafe { GetModuleHandleW(module_name_wide.as_ptr()) as *mut c_void }
}

/// Retrieves the address of a specified procedure within a module.
///
/// This function uses the `GetProcAddress` function from the Windows API to obtain the address of a
/// specified procedure within a module. The procedure address can then be used to call the procedure
/// directly from Rust code.
///
/// # Parameters
///
/// * `module_handle`: A raw pointer to the module's handle. This can be obtained using the
///   `get_module_handle` function.
/// * `proc_name`: A string representing the name of the procedure to retrieve.
///
/// # Return Value
///
/// Returns a raw pointer to the specified procedure. This pointer can be safely cast to the desired
/// function signature and used to call the procedure directly.
///
/// If the specified procedure is not found within the module, the function returns `null`.
pub fn get_proc_address(module_handle: *mut c_void, proc_name: &str) -> *mut c_void {
    let proc_name_cstr = CString::new(proc_name).expect("CString::new failed");
    unsafe { GetProcAddress(module_handle as *mut _, proc_name_cstr.as_ptr()) as *mut c_void }
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
pub fn get_module_info(module_handle: *mut c_void) -> Option<MODULEINFO> {
    let mut module_info =
        MODULEINFO { lpBaseOfDll: null_mut(), SizeOfImage: 0, EntryPoint: null_mut() };

    if unsafe {
        GetModuleInformation(
            GetCurrentProcess(),
            module_handle as *mut _,
            &mut module_info,
            std::mem::size_of::<MODULEINFO>() as u32,
        )
    } != 0
    {
        Some(module_info)
    } else {
        None
    }
}

/// Searches for a pattern within a module's memory.
///
/// This function takes a module handle and a pattern string as input.
/// The pattern string consists of hexadecimal bytes separated by spaces,
/// with "??" representing a wildcard that matches any byte.
/// The function searches for the pattern within the module's memory and returns the address of the first occurrence.
///
/// # Parameters
///
/// * `module_handle`: A raw pointer to the module's handle. This can be obtained using the `get_module_handle` function.
/// * `pattern`: A string representing the pattern to search for.
///
/// # Return Value
///
/// Returns `Some(address)` if the pattern is found, where `address` is the memory address of the first occurrence.
/// Returns `None` if the pattern is not found or if an error occurs during pattern parsing.
pub fn pattern_search(module_handle: *mut c_void, pattern: &str) -> Option<usize> {
    // Split the pattern string into bytes and handle wildcards
    let pattern_bytes: Result<Vec<Option<u8>>, _> =
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

    let pattern_bytes = match pattern_bytes {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to parse pattern: {}", e);
            return None;
        }
    };

    let module_info = match get_module_info(module_handle) {
        Some(info) => info,
        None => return None,
    };

    let base_address = module_info.lpBaseOfDll as *const u8;
    let size = module_info.SizeOfImage as usize;

    unsafe {
        let module_memory = slice::from_raw_parts(base_address, size);

        for i in 0..module_memory.len() - pattern_bytes.len() {
            if pattern_bytes
                .iter()
                .enumerate()
                .all(|(j, &b)| b.map_or(true, |b| module_memory[i + j] == b))
            {
                return Some(base_address.add(i) as usize);
            }
        }
    }

    None
}

/// Retrieves the address of a specific interface within a module.
///
/// # Parameters
///
/// * `module_handle`: A raw pointer to the module's handle. This can be obtained using the `get_module_handle` function.
/// * `interface_name`: A string representing the name of the interface to retrieve.
///
/// # Return Value
///
/// Returns a raw pointer to the interface if found, or `null` if the interface is not found.
/// The returned pointer can be safely cast to the desired interface type.
pub fn get_interface(module_handle: *mut c_void, interface_name: &str) -> *const usize {
    let function: unsafe extern "C" fn(
        name: *const c_char,
        return_code: *const c_int,
    ) -> *const c_void = unsafe { transmute(get_proc_address(module_handle, "CreateInterface")) };

    let interface_name_cstr = CString::new(interface_name).expect("CString::new failed");
    unsafe { function(interface_name_cstr.as_ptr(), null_mut()) as *const usize }
}
