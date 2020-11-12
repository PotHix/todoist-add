use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn add(token: *mut c_char, content: *mut c_char) -> *mut c_char {
    let c_str_token = unsafe { CStr::from_ptr(token) };
    let str_token = c_str_token.to_str().unwrap();

    let c_str_content = unsafe { CStr::from_ptr(content) };
    let str_content = c_str_content.to_str().unwrap();

    let res = super::add(str_token.to_string(), str_content.to_string());

    let s = CString::new(res.unwrap()).unwrap();
    return s.into_raw();
}

#[no_mangle]
pub extern "C" fn string_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
