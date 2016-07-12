extern crate tempdir;

use std::env;
use std::path::Path;

use self::tempdir::TempDir;

use proton_cli;

use super::rsa_keys::{self, TestKey};


/// Creates a temporary directory to run a test out of
pub fn setup() -> TempDir {
    TempDir::new("proton_cli_tests").unwrap()
}

/// Creates a temporary directory, initializes a project in it,
/// and changes the current directory to it
/// Returns the path to the temp directory 
pub fn setup_init_cd() -> TempDir {
    let root_dir = setup();
    
    {
        let root = root_dir.path();
        let root_pub_key = rsa_keys::get_test_key(TestKey::RootKeyPub);

        let _ = proton_cli::initialize_project(root, &root_pub_key)
            .expect("Error initializing project");

        // Move into temp directory (new_user assumes it is run in project directory)
        assert!(env::set_current_dir(&root).is_ok());
    }

    root_dir
}

/// Creates a key file for a new user,
/// then tries to add the user to the project
pub fn try_new_user(
    admin_key_path: &Path,
    root_path: &Path,
    user_name: &str,
    key_name: &str,
    key: TestKey
) {

    let user_key_path = super::make_key_file(&root_path, &key_name, key);

    let _ = match proton_cli::new_user(&admin_key_path, &user_key_path.as_path(), &user_name) {
        Ok(_) => (),
        Err(e) => panic!("{}", e.to_string()),
    };

    super::assert_user_added(user_key_path.as_path(), &user_name);

    super::assert_repo_no_modified_files(&root_path);
}

/// Attempts to make a new sequence with the given name and music file
/// Panics if error thrown
pub fn try_make_sequence(admin_key_path: &Path, name: &str, music_file: &str) {
    let music_file_path = super::get_music_file_path(music_file);

    let _ = match proton_cli::new_sequence(&admin_key_path, &name, &music_file_path.as_path()) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    };
}
