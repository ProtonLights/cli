extern crate proton_cli;
extern crate tempdir;

mod common;

use std::path::PathBuf;

use common::rsa_keys::TestKey;
use common::setup;


/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
fn works_with_new_and_existing_protonfile() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);

    setup::try_new_user(
        &root_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);
    setup::try_new_user(
        &root_key_path.as_path(),
        root.path(),
        "Test User 2",
        "b.pub",
        TestKey::GoodKey2Pub);
}

#[test]
#[should_panic(expected = "entity not found")]
fn fails_with_nonexistent_root_key_path() {
    let root = setup::setup_init_cd();
    let root_key_path = PathBuf::from("nonexistent");

    setup::try_new_user(
        &root_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);
}

#[test]
#[should_panic(expected = "SSL error")]
fn fails_with_invalid_admin_key_format() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPub);

    setup::try_new_user(
        &root_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);
}

#[test]
#[should_panic(expected = "Unauthorized action")]
fn fails_with_admin_no_privileges() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);

    // Create key files
    let user_key_path = common::make_key_file(&root.path(), "a.pub", TestKey::GoodKeyPub);
    let user_priv_key_path = common::make_key_file(&root.path(), "a.pem", TestKey::GoodKeyPem);
    let user2_key_path = common::make_key_file(&root.path(), "b.pub", TestKey::GoodKey2Pub);

    // Add new user to project
    let _ = match proton_cli::new_user(&root_key_path, &user_key_path, "userA") {
        Ok(u) => u,
        Err(e) => panic!("{}", e.to_string()),
    };

    // Assert that user was added
    common::assert_user_added(user_key_path.as_path(), "userA");

    // Check that commit was made
    common::assert_repo_no_modified_files(&root.path());

    // Create second user with first user as admin
    let _ = match proton_cli::new_user(
        &user_priv_key_path,
        &user2_key_path,
        "userB"
    ) {
        Ok(u) => u,
        Err(e) => panic!("{}", e.to_string()),
    };
}

#[test]
#[should_panic(expected = "not found")]
fn fails_with_a_nonexistent_protonfile() {
    // Don't initialize project (no protonfile created)
    let root_dir = setup::setup();
    let root_key_path = common::make_key_file(&root_dir.path(), "root.pem", TestKey::RootKeyPem);

    setup::try_new_user(
        root_key_path.as_path(),
        root_dir.path(),
        "Username",
        "a.pub",
        TestKey::GoodKeyPub);
}

#[test]
#[should_panic(expected = "Error adding user")]
fn fails_with_nonexistent_user_key_path() {
    let root = setup::setup_init_cd();

    let user_key_path = root.path().join("nonexistent");
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);

    match proton_cli::new_user(&root_key_path.as_path(), &user_key_path.as_path(), "Username") {
        Ok(_) => (),
        Err(_) => panic!("Error adding user"),
    };
}

#[test]
#[should_panic(expected = "Public key is invalid")]
fn fails_with_non_pem_key() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);

    setup::try_new_user(
        root_key_path.as_path(),
        root.path(),
        "Test User",
        "bad_pub_key.pub",
        TestKey::BadPubKeyPub);
}

/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
#[should_panic(expected = "Duplicate user")]
fn fails_with_duplicate_user_key() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);

    setup::try_new_user(
        root_key_path.as_path(),
        root.path(),
        "Test User 1",
        "a.pub",
        TestKey::GoodKeyPub);
    setup::try_new_user(
        root_key_path.as_path(),
        root.path(),
        "Test User 2",
        "b.pub",
        TestKey::GoodKeyPub);
}

/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
#[should_panic(expected = "Duplicate user")]
fn fails_with_duplicate_user_name() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);

    setup::try_new_user(
        root_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);
    setup::try_new_user(
        root_key_path.as_path(),
        root.path(),
        "Test User",
        "b.pub",
        TestKey::GoodKey2Pub);
}

