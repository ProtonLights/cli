extern crate proton_cli;
extern crate tempdir;

pub mod rsa_keys;

use std::fs::File;
use std::io::{Write};
use std::path::{Path, PathBuf};

use tempdir::TempDir;


/// Creates a key file at the given location
/// Returns the path to the key file
pub fn make_key_file<P: AsRef<Path>>(
    root_dir: P,
    file_name: &str,
    test_key: rsa_keys::TestKey
) -> PathBuf {

    let mut key_path = PathBuf::new();
    key_path.push(root_dir);
    key_path.push(file_name);

    let file_content = rsa_keys::get_test_key(test_key);
    File::create(&key_path)
        .and_then(|mut file| write!(file, "{}\n", file_content))
        .expect("Error creating key file");

    key_path
}

/// Creates a temporary directory to run a test out of
pub fn setup() -> TempDir {
    TempDir::new("proton_cli_tests").unwrap()
}