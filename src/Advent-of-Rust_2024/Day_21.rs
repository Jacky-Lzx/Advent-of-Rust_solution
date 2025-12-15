use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::{fs::File, path::PathBuf};

use std::time::{SystemTime, UNIX_EPOCH};

fn generate_std_filename(ext: &str) -> String {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let microsecs = duration.subsec_micros();

    format!("file_{}.{}", microsecs, ext)
}

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        // Your code here...
        let file_name = generate_std_filename("tmp");
        let file_path = std::env::temp_dir().join(file_name);
        let file = File::create(&file_path)?;

        Ok(Self { file_path, file })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        // Your code here...
        let mut file = OpenOptions::new().write(true).open(&self.file_path)?;
        file.write_all(data)?;
        Ok(())
    }

    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        // Your code here...
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.file_path);
    }
}

fn main() {
    println!("Hello world");
}
