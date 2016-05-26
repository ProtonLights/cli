extern crate proton_cli;
extern crate tempdir;

mod common;

use std::env;

use common::rsa_keys::TestKey;


/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
fn works_with_new_and_existing_protonfile() {
    // Make temp directory to work out of
    let root_dir = common::setup();
    let root = root_dir.path();

    // Make new project in temp directory
    let _ = proton_cli::initialize_project(&root)
        .expect("Error initializing project");

    // Make key files for users
    let key_path_a = common::make_key_file(root, "a.pub", TestKey::GoodKeyPub);
    let key_path_b = common::make_key_file(root, "b.pub", TestKey::GoodKey2Pub);

    // Move into temp directory (new_user assumes it is run in project directory)
    assert!(env::set_current_dir(&root).is_ok());

    let user_name = "Test User";
    let user_name2 = "Test User 2";

    // Add new user to project
    let _ = proton_cli::new_user(&key_path_a.as_path(), &user_name)
        .expect("Error adding user");

    // Assert that user was added
    common::assert_user_added(key_path_a.as_path(), "Test User");

    // Now try adding another user
    let _ = proton_cli::new_user(&key_path_b.as_path(), &user_name2)
        .expect("Error adding user 2");

    // Assert that both users exist
    common::assert_user_added(key_path_a.as_path(), &user_name);
    common::assert_user_added(key_path_b.as_path(), &user_name2);
}

#[test]
#[should_panic(expected = "Error adding user")]
fn fails_with_a_nonexistent_protonfile() {
    let root_dir = common::setup();
    let root = root_dir.path();

    // Make key file, but don't initialize project
    let key_path = common::make_key_file(root, "a.pub", TestKey::GoodKeyPub);

    match proton_cli::new_user(&key_path.as_path(), "Username") {
        Ok(_) => (),
        Err(_) => panic!("Error adding user"),
    };
}

#[test]
#[should_panic(expected = "Error adding user")]
fn fails_with_nonexistent_key_path() {
    let root_dir = common::setup();
    let root = root_dir.path();

    let _ = proton_cli::initialize_project(&root)
        .expect("Error initializing project");
    
    let key_path = root.join("nonexistent");

    // Move into temp directory (new_user assumes it is run in project directory)
    assert!(env::set_current_dir(&root).is_ok());

    match proton_cli::new_user(&key_path.as_path(), "Username") {
        Ok(_) => (),
        Err(_) => panic!("Error adding user"),
    };
}

#[test]
#[should_panic(expected = "Public key is invalid")]
fn fails_with_non_pem_key() {
    let root_dir = common::setup();
    let root = root_dir.path();

    let _ = proton_cli::initialize_project(&root)
        .expect("Error initializing project");

    let key_path = common::make_key_file(&root, "bad_pub_key.pub", TestKey::BadPubKeyPub);

    // Move into temp directory (new_user assumes it is run in project directory)
    assert!(env::set_current_dir(&root).is_ok());

    // Add new user to project
    match proton_cli::new_user(&key_path.as_path(), "Test User") {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };

}

/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
#[should_panic(expected = "Error adding user 2")]
fn fails_with_duplicate_user_key() {
    let root_dir = common::setup();
    let root = root_dir.path();

    let _ = proton_cli::initialize_project(&root)
        .expect("Error initializing project");
    
    let key_path = common::make_key_file(root, "a.pub", TestKey::GoodKeyPub);

    // Move into temp directory (new_user assumes it is run in project directory)
    assert!(env::set_current_dir(&root).is_ok());

    // Add new user to project
    let _ = proton_cli::new_user(&key_path.as_path(), "Test User")
        .expect("Error adding user");

    // Assert that user was added
    common::assert_user_added(key_path.as_path(), "Test User");

    // Now try adding another user with the same key
    let _ = proton_cli::new_user(&key_path.as_path(), "Test User 2")
        .expect("Error adding user 2");

    panic!("Should not get to here");
}

/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
#[should_panic(expected = "Error adding second user")]
fn fails_with_duplicate_user_name() {
    let root_dir = common::setup();
    let root = root_dir.path();

    let _ = proton_cli::initialize_project(&root)
        .expect("Error initializing project");
    
    let key_path_a = common::make_key_file(root, "a.pub", TestKey::GoodKeyPub);
    let key_path_b = common::make_key_file(root, "b.pub", TestKey::GoodKey2Pub);

    // Move into temp directory (new_user assumes it is run in project directory)
    assert!(env::set_current_dir(&root).is_ok());

    // Add new user to project
    let _ = proton_cli::new_user(&key_path_a.as_path(), "Test User")
        .expect("Error adding user");

    // Assert that user was added
    common::assert_user_added(key_path_a.as_path(), "Test User");

    // Now try adding another user with the same key
    let _ = proton_cli::new_user(&key_path_b.as_path(), "Test User")
        .expect("Error adding second user");

    panic!("Should not get to here");
}

