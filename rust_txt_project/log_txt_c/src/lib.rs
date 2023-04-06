use std::ffi::{CStr, CString};
use std::os::raw::c_char;
// use std::fs::{OpenOptions, File};
// use std::io::{Error, Write, Read, BufReader};

use rust_txt_core::{append_to_file as append_to_file_inner, read_from_file as read_from_file_inner};


#[no_mangle]
pub extern "C" fn append_to_file(file_name: *const c_char, content: *const c_char) -> i32 {
    let file_name = unsafe { CStr::from_ptr(file_name).to_string_lossy().into_owned() };
    let content = unsafe { CStr::from_ptr(content).to_string_lossy().into_owned() };
    match append_to_file_inner(&file_name, &content) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

#[no_mangle]
pub extern "C" fn read_from_file(file_name: *const c_char) -> *mut c_char {
    let file_name = unsafe { CStr::from_ptr(file_name).to_string_lossy().into_owned() };
    match read_from_file_inner(&file_name) {
        Ok(contents) => CString::new(contents).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

