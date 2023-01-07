use std::fs;
use std::io;

pub struct DataInterface {
    pub data: Vec<u8>,
}

impl DataInterface {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn read_from_file(&mut self, file_path: &str) -> Result<(), io::Error> {
        let data = fs::read(file_path)?;
        self.data = data;
        Ok(())
    }

    pub fn write_to_file(&self, file_path: &str) -> Result<(), io::Error> {
        fs::write(file_path, &self.data)?;
        Ok(())
    }
}
