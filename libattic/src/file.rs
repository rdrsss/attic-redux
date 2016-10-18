

/// Contains metadata to a file in the file system.
pub struct File {
    path: String,
}

impl File {
    pub fn new(path: String) -> File {
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
}

