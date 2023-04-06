use pyo3::prelude::*;
use rust_txt_core::{append_to_file as append_to_file_core, read_from_file as read_from_file_core};


#[pyfunction]
fn append_to_file(file_name: &str, content: &str) -> PyResult<()> {
    match append_to_file_core(&file_name, &content){
        Ok(()) => Ok(()),
        Err(_) => todo!()
        // Err() => PyErr()
    }
}

#[pyfunction]
fn read_from_file(file_name: &str) -> PyResult<String> {
    match read_from_file_core(&file_name){
        Ok(contents) => Ok(contents),
        Err(_) => todo!()
    }
}

#[pymodule]
fn log_file(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(append_to_file, m)?)?;
    m.add_function(wrap_pyfunction!(read_from_file, m)?)?;
    Ok(())
}