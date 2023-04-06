use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::fs::{OpenOptions, File};
use std::io::{Error, Write, Read, BufReader};

#[no_mangle]
pub extern "C" fn append_to_file(file_name: *const c_char, content: *const c_char) -> i32 {
    let file_name = unsafe { CStr::from_ptr(file_name).to_string_lossy().into_owned() };
    let content = unsafe { CStr::from_ptr(content).to_string_lossy().into_owned() };
    match append_to_file_inner(&file_name, &content) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

fn append_to_file_inner(file_name: &str, content: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)?;

    file.write_all(content.as_bytes())?;
    file.write_all("\n".as_bytes())?;

    Ok(())
}

#[no_mangle]
pub extern "C" fn read_from_file(file_name: *const c_char) -> *mut c_char {
    let file_name = unsafe { CStr::from_ptr(file_name).to_string_lossy().into_owned() };
    match read_from_file_inner(&file_name) {
        Ok(contents) => CString::new(contents).unwrap().into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

fn read_from_file_inner(file_name: &str) -> Result<String, Error> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}