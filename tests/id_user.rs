extern crate proton_cli;
extern crate tempdir;

use tempdir::TempDir;

use proton_cli::{Project, User, utils};


#[test]
fn works_with_valid_keys() {

}


/// Creates a temporary directory to run a test out of
fn setup() -> TempDir {
    TempDir::new("proton_cli_tests").unwrap()
}
