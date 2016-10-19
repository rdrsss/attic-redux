
use std;

/// Contains metadata to a file in the file system.
pub struct File {
    path: String,
}

impl File {
    pub fn new(path: String) -> File {
        // Check if file is valid.
        // Gather metadata about file. 
        File {
            path: path,
        }
    }

    pub fn get_path(&self) -> &String {
        return &self.path;
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn get_filesize(&self) -> u64 {
        let result = std::fs::metadata(&self.path);
        assert!(result.is_ok());
        let metadata = result.unwrap();
        return metadata.len();
    }
}

