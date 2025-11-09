use crate::ControlHandle;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr;
use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::HWND;

pub const CUSTOM_ID_BEGIN: u32 = 10000;

pub fn check_hwnd(handle: &ControlHandle, not_bound: &str, bad_handle: &str) -> HWND {
    use winapi::um::winuser::IsWindow;

    if handle.blank() {
        panic!("{}", not_bound);
    }
    match handle.hwnd() {
        Some(hwnd) => match unsafe { IsWindow(hwnd) } {
            0 => {
                panic!(
                    "The window handle is no longer valid. This usually means the control was freed by the OS"
                );
            }
            _ => hwnd,
        },
        None => {
            panic!("{}", bad_handle);
        }
    }
}

pub fn to_utf16<'a>(s: &'a str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(s)
        .encode_wide()
        .chain(Some(0u16).into_iter())
        .collect()
}

/**
    Decode a raw utf16 string. Should be null terminated.
*/
pub fn from_utf16(s: &[u16]) -> String {
    use std::os::windows::ffi::OsStringExt;

    let null_index = s.iter().position(|&i| i == 0).unwrap_or(s.len());
    let os_string = OsString::from_wide(&s[0..null_index]);

    os_string
        .into_string()
        .unwrap_or("Decoding error".to_string())
}

/**
    Read a string from a wide char pointer. Undefined behaviour if [ptr] is not null terminated.
*/
#[cfg(feature = "winnls")]
pub unsafe fn from_wide_ptr(ptr: *mut u16, length: Option<usize>) -> String {
    use std::slice::from_raw_parts;

    let length = match length {
        Some(v) => v,
        None => {
            let mut length: isize = 0;
            unsafe {
                while *&*ptr.offset(length) != 0 {
                    length += 1;
                }
            }

            length as usize
        }
    };

    let array: &[u16] = unsafe { from_raw_parts(ptr, length) };
    from_utf16(array)
}

#[cfg(any(feature = "file-dialog", feature = "winnls"))]
pub unsafe fn os_string_from_wide_ptr(ptr: *mut u16, length: Option<usize>) -> OsString {
    use std::os::windows::ffi::OsStringExt;
    use std::slice::from_raw_parts;

    let length = match length {
        Some(v) => v,
        None => {
            let mut length: isize = 0;
            unsafe {
                while *&*ptr.offset(length) != 0 {
                    length += 1;
                }
            }

            length as usize
        }
    };

    let array: &[u16] = unsafe { from_raw_parts(ptr, length) };
    OsString::from_wide(array)
}

/**
    Return a formatted output of the last system error that was raised.

    (ERROR ID, Error message localized)
*/
#[allow(unused)]
pub unsafe fn get_system_error() -> (DWORD, String) {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::winbase::{FORMAT_MESSAGE_FROM_SYSTEM, FormatMessageW};
    use winapi::um::winnt::{LANG_NEUTRAL, MAKELANGID, SUBLANG_DEFAULT};

    let code = unsafe { GetLastError() };
    let lang = MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT) as DWORD;
    let mut buf: Vec<u16> = Vec::with_capacity(1024);
    unsafe {
        buf.set_len(1024);
    }
    unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM,
            ptr::null(),
            code,
            lang,
            buf.as_mut_ptr(),
            1024,
            ptr::null_mut(),
        );
    }

    let end = buf.iter().position(|&i| i == 0).unwrap_or(1024);
    let error_message = OsString::from_wide(&buf[..end])
        .into_string()
        .unwrap_or("Error while decoding system error message".to_string());

    (code, error_message)
}

/// Converts a wide C string pointer to a string.
///
/// # Arguments
///
/// * `ptr`: Pointer to a null-terminated Rust string.
///
/// returns: String
pub fn cwstr_to_str(ptr: isize) -> String {
    if ptr == 0 {
        return "".to_string();
    }

    let wide_ptr = ptr as *const u16;

    let mut len = 0;
    while unsafe { *wide_ptr.add(len) } != 0 {
        len += 1;
    }

    let slice = unsafe { std::slice::from_raw_parts(wide_ptr, len) };

    let os_string = OsString::from_wide(slice);
    os_string.to_string_lossy().into_owned()
}
