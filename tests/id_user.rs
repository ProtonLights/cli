extern crate proton_cli;
extern crate tempdir;

use tempdir::TempDir;

use proton_cli::{Project, User, utils};
mod common;


#[test]
fn works_with_valid_keys() {
    let root_dir = common::setup();
    let root = root_dir.path();

    let _ = proton_cli::initialize_project(&root)
        .expect("Error initializing project");
}

