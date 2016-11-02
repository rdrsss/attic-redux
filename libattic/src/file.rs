
use std;

pub enum AtticFileError {
    Metadata,
    PathExists
}

/// Implement this trait to be able to println! error enum
impl std::fmt::Display for AtticFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// The File struct maps a libattic file to a file within the file system.
///     * Keeps track of file metadata
///     * File hash
pub struct File {
    path: String,
    hash: String, 
}

impl File {
    /// Builds a new File struct or returns an error string.
    pub fn new(path: String) -> Result<File, AtticFileError> {
        // Build empty file
        let mut f = File {
            path : path,
            hash : "".to_string(),
        };
        // Sync libattiac file to local file and return.
        match f.sync() {
            Ok(()) => Ok(f),
            Err(e) => Err(e),
        }
    }

    /// sync - syncs file meta data to to libattic file object.
    ///     * Checks if file is valid.
    ///     * Runs rolling hash (sha256)
    ///     * Can be run at any time to any path to update file object.
    pub fn sync(&mut self) -> Result<(), AtticFileError> { 
        // Get metadata
        if !std::path::Path::new(&self.path).exists() {
            return Err(AtticFileError::PathExists);
        }

        let metadata;
        match std::fs::metadata(&self.path) {
            Ok(v) => metadata = v,
            Err(e) => return Err(AtticFileError::Metadata),
        }
        // Check if file is valid.

        // Gather metadata about file. 
        // Generate hash.
        //
        Ok(())
    }

    /// Updates the filepath and re-syncs the File.
    pub fn update_path(&mut self, path: String) -> Result<(), AtticFileError> {
        self.path = path;
        match self.sync() {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
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

    fn generate_hash(&mut self, path: String) {
        // TODO :: this
        //
        // - Open file
        // - Run rolling hash (sha256)
    }
}

