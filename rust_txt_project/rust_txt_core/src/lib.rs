use std::fs::{OpenOptions, File};
use std::io::{Error, Write, Read, BufReader};

pub fn append_to_file(file_name: &str, content: &str) -> Result<(), Error> {
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

pub fn read_from_file(file_name: &str) -> Result<String, Error> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_to_file() {
        // Test appending to a new file
        let file_name = "test1.txt";
        let content = "Hello, world!";
        let result = append_to_file(file_name, content);
        assert!(result.is_ok());

        // Test appending to an existing file
        let content2 = "Second line.";
        let result2 = append_to_file(file_name, content2);
        assert!(result2.is_ok());

        // Test appending with invalid file name
        let invalid_file_name = "/some/invalid/path.txt";
        let result3 = append_to_file(invalid_file_name, content);
        assert!(result3.is_err());
        std::fs::remove_file("test1.txt").unwrap();
    }

    #[test]
    fn test_read_from_file() {
        // Test reading from an existing file
        let file_name = "test2.txt";
        let content = "Hello, world!";
        
        append_to_file(file_name, content);
        let result = read_from_file(file_name);
        
        assert!(result.is_ok());
        
        assert_eq!(result.unwrap(), "Hello, world!\n");
        std::fs::remove_file("test2.txt").unwrap();
    }
}
