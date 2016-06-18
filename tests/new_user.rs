extern crate proton_cli;
extern crate tempdir;

use std::path::Path;

mod common;

use common::rsa_keys::TestKey;


/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
fn works_with_new_and_existing_protonfile() {
    let root = common::setup_init_cd();

    try_add_user(root.path(), "Test User", "a.pub", TestKey::GoodKeyPub);
    try_add_user(root.path(), "Test User 2", "b.pub", TestKey::GoodKey2Pub);
}

#[test]
#[should_panic(expected = "entity not found")]
fn fails_with_a_nonexistent_protonfile() {
    // Don't initialize project (no protonfile created)
    let root_dir = common::setup();

    try_add_user(root_dir.path(), "Username", "a.pub", TestKey::GoodKeyPub);
}

#[test]
#[should_panic(expected = "Error adding user")]
fn fails_with_nonexistent_key_path() {
    let root = common::setup_init_cd();

    let key_path = root.path().join("nonexistent");

    match proton_cli::new_user(&key_path.as_path(), "Username") {
        Ok(_) => (),
        Err(_) => panic!("Error adding user"),
    };
}

#[test]
#[should_panic(expected = "Public key is invalid")]
fn fails_with_non_pem_key() {
    let root = common::setup_init_cd();
    try_add_user(root.path(), "Test User", "bad_pub_key.pub", TestKey::BadPubKeyPub);
}

/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
#[should_panic(expected = "Duplicate user")]
fn fails_with_duplicate_user_key() {
    let root = common::setup_init_cd();

    try_add_user(root.path(), "Test User 1", "a.pub", TestKey::GoodKeyPub);
    try_add_user(root.path(), "Test User 2", "b.pub", TestKey::GoodKeyPub);
}

/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
#[should_panic(expected = "Duplicate user")]
fn fails_with_duplicate_user_name() {
    let root = common::setup_init_cd();

    try_add_user(root.path(), "Test User", "a.pub", TestKey::GoodKeyPub);
    try_add_user(root.path(), "Test User", "b.pub", TestKey::GoodKey2Pub);
}

/// Creates a key file for a new user,
/// then tries to add the user to the project
/// Returns any errors received
fn try_add_user(
    root_path: &Path,
    user_name: &str,
    key_name: &str,
    key: TestKey
) {

    let key_path = common::make_key_file(&root_path, &key_name, key);

    // Add new user to project
    match proton_cli::new_user(key_path.as_path(), &user_name) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    };

    // Assert that user was added
    common::assert_user_added(key_path.as_path(), &user_name);

    common::assert_repo_no_modified_files(&root_path);
}
