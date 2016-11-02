extern crate libattic;
extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::io::Write;
//use std::fs::File;
//
//
// Note* Rust tests are run concurrently no sequentially 

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

fn create_test_file(filepath: String, stride: u32) {
    println!("Creating a test file ...");
    let result = std::fs::File::create(filepath);
    assert!(result.is_ok());
    let mut f = result.unwrap();
    for _ in 0..stride {
        assert!(f.write_all(b"test test test").is_ok());
    }
}

fn delete_test_file(filepath: String) {
    let result = std::fs::remove_file(filepath);

    if !result.is_ok() {
        println!("Failed to delete file {}", result.is_ok())
    }
}

#[test]
fn file_sentinel_rolling_hash() {
    let filepath = "sentinel_test";
    // Create a test file
    create_test_file(filepath.to_string(), 1);
    // Read it in and setup meta data
    let local_file;
    match libattic::file::File::new(filepath.to_string()) {
        Ok(f) => local_file = f,
        Err(e) => {
            println!(" Failed to create file {}", e);
            assert!(false);
            return;
        }
    }
    //assert!(local_file.get_filesize());
    println!("File size: {}", local_file.get_filesize());
    // Delete test file
    delete_test_file(filepath.to_string());
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

#[test]
fn generate_file_hash() {
    // Create a largish test file
    create_test_file("hash_file_test".to_string(), 100);
    // Create a lib attic file object
    // Generate hash and metadata about given file.
    // Delete test file
    delete_test_file("hash_file_test".to_string());
}







