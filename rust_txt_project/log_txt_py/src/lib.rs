use pyo3::prelude::*;
use std::fs::{OpenOptions, File};
use std::io::{Error, Write, Read, BufReader};
use chrono::prelude::*;


#[pyfunction]
fn append(file_name: &str, content: &str) -> PyResult<()> {
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

#[pyfunction]
fn read(file_name: &str) -> PyResult<String> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

#[pymodule]
fn log_file(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(append, m)?)?;
    m.add_function(wrap_pyfunction!(read, m)?)?;
    Ok(())
}