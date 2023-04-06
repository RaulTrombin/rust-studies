use std::fs::{OpenOptions, File};
use std::io::{Error, Write, Read, BufReader};

fn append_to_file(file_name: &str, content: &str) -> Result<(), Error> {
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

fn read_from_file(file_name: &str) -> Result<String, Error> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}