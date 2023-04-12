use std::ffi::{CStr, CString};
use std::os::raw::c_char;
// use std::fs::{OpenOptions, File};
// use std::io::{Error, Write, Read, BufReader};

use rust_txt_core::{append_to_file as append_to_file_core, read_from_file as read_from_file_core};

#[no_mangle]
pub extern "C" fn append_to_file(file_name: *const c_char, content: *const c_char) -> i32 {
    let file_name = unsafe {
        CStr::from_ptr(file_name)
            .to_str()
            .expect("Invalid UTF-8 sequence in file_name")
    };
    let content = unsafe {
        CStr::from_ptr(content)
            .to_str()
            .expect("Invalid UTF-8 sequence in content")
    };
    match append_to_file_core(file_name, content) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

#[no_mangle]
pub extern "C" fn read_from_file(file_name: *const c_char) -> *mut c_char {
    let file_name = unsafe {
        CStr::from_ptr(file_name)
            .to_str()
            .expect("Invalid UTF-8 sequence in file_name")
    };
    match read_from_file_core(file_name) {
        Ok(contents) => CString::new(contents).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn add(a: i32, b:i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::add;
    use crate::append_to_file;
    use crate::read_from_file;
    use std::ffi::CStr;
    use std::ffi::CString;

    #[test]
    fn add_positive_numbers() {
        assert_eq!(add(2,2), 4);
    }

    #[test]
    fn add_negative_numbers() {
        assert_eq!(add(-2,-3), -5);
    }
    
    #[test]
    fn append_to_file_test() {
        let file_name = CString::new("test.txt").unwrap().into_raw();
        let content = CString::new("hello world").unwrap().into_raw();

        let result = unsafe { append_to_file(file_name, content) };

        assert_eq!(result, 0);
        std::fs::remove_file("test.txt").unwrap();
    }

    #[test]
    fn read_from_file_test() {
        let file_name = CString::new("testr.txt").unwrap().into_raw();
        let content = CString::new("hello world").unwrap().into_raw();
        let expected = CString::new("hello world\n").unwrap();
        let result = unsafe { append_to_file(file_name, content) };

        assert_eq!(result, 0);
        
        let result = unsafe { read_from_file(file_name) };
        let result_str = unsafe { CStr::from_ptr(result).to_str().unwrap() };
        

        assert_eq!(result_str, expected.to_str().unwrap());

        // clean up the file after the test
        std::fs::remove_file("testr.txt").unwrap();
}
    // #[test]
    // fn check_txt() {
    //     // assert_eq!(append_to_file("c","c"), null_mut);
    //     assert_eq!(read_from_file("c"), 'c'.);        
    // }
}