use std::io::Write;

static TEST_FILENAME : &'static str = "test_file";

#[cfg(test)]
mod tests {
    #[test]
    fn second_it_works() {
        println!("Libattic hello!");
    }

    #[test]
    fn wo_wot() {
        println!("sags")
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
    assert!(result.is_ok());
}

#[test]
fn file_sentinel_rolling_hash() {
    // Create a test file
    create_test_file();
    // Read it in and setup meta data
    let mut local_file = libattic::file_sentinel::File;
    // Delete test file
    delete_test_file();
}

#[test]
fn yet_another_one() {
    println!("I think i get it now");
}
