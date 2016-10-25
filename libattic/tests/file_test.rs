extern crate libattic;
extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::io::Write;

static TEST_FILENAME : &'static str = "test_file";

#[cfg(test)]
mod tests {
    #[test]
    fn second_it_works() {
        println!("Libattic hello!");
    }
}

// Create a file.
// Delete created file.
// Read file in
// Run a checksum on it, some sort of rolling hash.

fn create_test_file() {
    println!("Creating a test file ...");
    let result = std::fs::File::create(TEST_FILENAME);
    assert!(result.is_ok());
    let mut f = result.unwrap();
    assert!(f.write_all(b"test test test").is_ok());
}

fn delete_test_file() {
    let result = std::fs::remove_file(TEST_FILENAME);

    if (!result.is_ok()) {
        println!("Failed to delete file {}", result.is_ok())
    }
}

#[test]
fn file_sentinel_rolling_hash() {
    // Create a test file
    create_test_file();
    // Read it in and setp meta data
    let local_file = libattic::file::File::new(TEST_FILENAME.to_string()); // {path:"path".to_string()};
    //assert!(local_file.get_filesize());
    println!("File size: {}",local_file.get_filesize());
    // Delete test file
    delete_test_file();
}


#[test]
fn sha_256_hash_test() {
    // Create Sha256 Object
    let mut hasher = Sha256::new();
    // Write input message
    // A stream can be input like this in real time 
    hasher.input_str("test test test");
    hasher.input_str("1234");
    hasher.input_str("asdkghjaskdjhgkashdgkjhaskgh");
    // Read hash digest
    let hex = hasher.result_str();

    println!("Test hash : {}", hex)
}
