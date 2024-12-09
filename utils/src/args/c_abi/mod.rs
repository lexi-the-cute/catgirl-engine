use core::ffi::{c_char, c_int};

/// Parse arguments from C and send to the Clap library
///
/// # Safety
///
/// This only checks if argv is null,
/// it does not verify that argv points to valid data
///
/// # Panics
///
/// May panic if one of the arguments contains invalid UTF-8 data
#[must_use]
pub unsafe fn parse_args_from_c(
    arg_count: c_int,
    arg_vector_pointer: *const *const *const c_char,
) -> Option<Vec<String>> {
    use core::ffi::CStr;

    // Check if argv_pointer is null
    if arg_vector_pointer.is_null() {
        return None;
    }

    // Cast back to *const *const c_char so we can operate on it
    //  now that we passed the Safe API Boundary/Barrier
    let arg_vector: *const *const c_char = arg_vector_pointer.cast::<*const c_char>();

    // Check if argv is null
    if arg_vector.is_null() {
        return None;
    }

    // Parse array out of argv
    #[allow(clippy::cast_sign_loss)]
    let c_args: &[*const c_char] =
        unsafe { std::slice::from_raw_parts(arg_vector, arg_count as usize) };

    let mut args: Vec<String> = vec![];
    for &arg in c_args {
        let c_str: &CStr = unsafe { CStr::from_ptr(arg) };

        // This can cause panic
        let str_slice: &str = c_str.to_str().unwrap();

        args.push(str_slice.to_string());
    }

    Some(args)
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    #[test]
    fn test_parsing_args_from_c() {
        use super::*;

        // Null pointer should be disregarded (e.g. return is None)
        unsafe {
            assert_eq!(parse_args_from_c(999, core::ptr::null()), None);
        }

        // Valid argument passed in (e.g. return is vec!["hello"])
        unsafe {
            // Create C String
            let arg_one: CString = CString::new("hello").unwrap();
            let arg_one_ptr: *const c_char = arg_one.as_ptr();

            // Add C String to array
            let argv: [*const c_char; 1] = [arg_one_ptr];

            // Test Parser
            assert_eq!(
                parse_args_from_c(
                    i32::try_from(argv.len()).unwrap(),
                    argv.as_ptr().cast::<*const *const c_char>()
                ),
                Some(vec!["hello".to_string()])
            );
        }
    }
}
