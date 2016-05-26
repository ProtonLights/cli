extern crate proton_cli;
extern crate tempdir;

mod common;

use std::env;
use std::path::Path;

use proton_cli::{Project, User, utils};
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

    // Add new user to project
    let _ = proton_cli::new_user(&key_path_a.as_path(), String::from("Test User"))
        .expect("Error adding user");

    // Assert that user was added
    assert_user_added(key_path_a.as_path(), "Test User");

    // Now try adding another user
    let _ = proton_cli::new_user(&key_path_b.as_path(), String::from("Test User 2"))
        .expect("Error adding user 2");

    // Assert that both users exist
    assert_user_added(key_path_a.as_path(), "Test User");
    assert_user_added(key_path_b.as_path(), "Test User 2");
}

#[test]
#[should_panic(expected = "Error adding user")]
fn fails_with_a_nonexistent_protonfile() {
    let root_dir = common::setup();
    let root = root_dir.path();

    // Make key file, but don't initialize project
    let key_path = common::make_key_file(root, "a.pub", TestKey::GoodKeyPub);

    match proton_cli::new_user(&key_path.as_path(), String::from("Username")) {
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

    match proton_cli::new_user(&key_path.as_path(), String::from("Username")) {
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
    match proton_cli::new_user(&key_path.as_path(), String::from("Test User")) {
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
    let _ = proton_cli::new_user(&key_path.as_path(), String::from("Test User"))
        .expect("Error adding user");

    // Assert that user was added
    assert_user_added(key_path.as_path(), "Test User");

    // Now try adding another user with the same key
    let _ = proton_cli::new_user(&key_path.as_path(), String::from("Test User 2"))
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
    let _ = proton_cli::new_user(&key_path_a.as_path(), String::from("Test User"))
        .expect("Error adding user");

    // Assert that user was added
    assert_user_added(key_path_a.as_path(), "Test User");

    // Now try adding another user with the same key
    let _ = proton_cli::new_user(&key_path_b.as_path(), String::from("Test User"))
        .expect("Error adding second user");

    panic!("Should not get to here");
}


/// Check if the public key at the given path exists and contains key_content,
/// and check to see that the user is in the project at the current directory's protonfile
fn assert_user_added<P: AsRef<Path>>(public_key_path: P, name: &str) {
    let pub_key_contents = utils::file_as_string(public_key_path)
        .expect("Error reading public key file");

    let project: Project = utils::read_protonfile(None::<P>)
        .expect("Error reading project");
        
    let u = User {
        name: name.to_string(),
        public_key: pub_key_contents,
    };
    assert_eq!(project.user_exists(&u), true);
}
