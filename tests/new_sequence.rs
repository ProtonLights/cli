extern crate proton_cli;
extern crate tempdir;
extern crate git2;

mod common;

use std::path::{Path, PathBuf};

use common::setup;
use proton_cli::utils;
use common::rsa_keys::TestKey;


#[test]
fn works_with_valid_path_and_name() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    setup::try_make_sequence(&root_key_path.as_path(), "New_Sequence", "Dissonance.ogg");

    // Make sure the calculated music duration is correct
    // and check that the sequence folder is named correctly
    match utils::read_protonfile(Some(&root.path())) {
        Ok(project) => {
            let sequence = &project.sequences[0];
            // Dissonance is 5 min, 4 sec
            assert_eq!(sequence.music_duration_sec, 304);
            assert_eq!(sequence.directory_name, "seq_New_Sequence");

            // Make sure section1 was created
            let mut section_path = PathBuf::from(&sequence.directory_name);
            section_path.push("New_Sequence_section1");
            let section_path = section_path;
            assert!(section_path.exists());

        },
        Err(e) => panic!(e.to_string()),
    };

    // Make sure changes were committed
    common::assert_repo_no_modified_files(&root.path());
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "entity not found")]
fn fails_with_nonexistent_private_key() {
    let root = setup::setup_init_cd();
    let root_key_path = Path::new("nonexistent");
    setup::try_make_sequence(&root_key_path, "New_Sequence", "Dissonance.ogg");
}

#[test]
#[should_panic(expected = "User not found")]
fn fails_with_no_user_private_key() {
    let root = setup::setup_init_cd();
    let key_path = common::make_key_file(&root.path(), "a.pem", TestKey::GoodKeyPem);
    setup::try_make_sequence(&key_path, "New_Sequence", "Dissonance.ogg");
}

#[test]
#[should_panic(expected = "Unauthorized action")]
fn fails_with_no_admin_private_key() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let key_path = common::make_key_file(&root.path(), "a.pem", TestKey::GoodKeyPem);

    setup::try_new_user(&root_key_path, &root.path(), "Test user", "a.pub", TestKey::GoodKeyPub);
    setup::try_make_sequence(&key_path, "New_Sequence", "Dissonance.ogg");
}

#[test]
#[should_panic(expected = "SSL error")]
fn fails_with_invalid_private_key() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::GoodKeyPub);
    setup::try_make_sequence(&root_key_path, "New_Sequence", "Dissonance.ogg");
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Unsupported file type")]
fn fails_with_invalid_file_extension() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    setup::try_make_sequence(&root_key_path.as_path(), "New_Sequence", "Dissonance.mp3");
}

#[test]
#[should_panic(expected = "Duplicate sequence detected, music file not copied")]
fn fails_with_duplicate_sequence_name() {
    let root = setup::setup_init_cd();

    let name = "New_Sequence";
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);

    setup::try_make_sequence(&root_key_path.as_path(), &name, "Dissonance.ogg");

    let music_file_path = common::get_music_file_path("GlorytotheBells.ogg");

    match proton_cli::new_sequence(&root_key_path.as_path(), &name, &music_file_path.as_path()) {
        Ok(_) => (),
        Err(_) => {
            // Make sure the second music file wasn't copied
            let dest_path = Path::new(&root.path()).join("GlorytotheBells.ogg");
            assert!(!dest_path.exists());
            panic!("Duplicate sequence detected, music file not copied");
        },
    };
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Sequence name had invalid characters")]
fn fails_with_invalid_sequence_name() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    setup::try_make_sequence(&root_key_path.as_path(), "New Sequence", "Dissonance.ogg");
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Music file not found")]
fn fails_with_nonexistent_music_file_path() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    setup::try_make_sequence(&root_key_path.as_path(), "New_Sequence", "nonexistent.ogg");
}

