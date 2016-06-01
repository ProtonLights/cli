extern crate proton_cli;
extern crate tempdir;
extern crate git2;

mod common;

use std::path::{Path, PathBuf};
use std::fs::File;

use proton_cli::utils;


#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_valid_music_file_path() {
    let root = common::setup_init_cd();

    let name = "New Sequence".to_string();

    let music_file_path = get_music_file_path("Dissonance.ogg");

    let _ = match proton_cli::new_sequence(&name, &music_file_path) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    };

    // Make sure the calculated music duration is correct within one second on either side
    match utils::read_protonfile(Some(&root.path())) {
        Ok(project) => {
            let sequence = &project.sequences[0];
            // Dissonance is 5 min, 4 sec
            assert_eq!(sequence.music_duration_sec, 304);
        },
        Err(e) => panic!(e.to_string()),
    };

    // Make sure changes were committed
    common::assert_commits_added(&root.path());
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Duplicate sequence")]
fn fails_with_duplicate_sequence_name() {
    let root = common::setup_init_cd();

    let name = "New Sequence".to_string();

    let music_file_path_a = get_music_file_path("Dissonance.ogg");
    let music_file_path_b = get_music_file_path("GlorytotheBells.ogg");

    match proton_cli::new_sequence(&name, &music_file_path_a) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    };

    match proton_cli::new_sequence(&name, &music_file_path_b) {
        Ok(_) => (),
        Err(e) => {
            // Make sure the second music file wasn't copied
            let dest_path = Path::new(&root.path()).join("GlorytotheBells.ogg");
            assert!(!dest_path.exists());
            panic!(e.to_string())
        },
    };

}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Duplicate music file")]
fn fails_with_duplicate_music_file() {
    let root = common::setup_init_cd();

    let name_a = "New Sequence".to_string();
    let name_b = "New Sequence 2".to_string();

    let music_file_path = get_music_file_path("Dissonance.ogg");

    match proton_cli::new_sequence(&name_a, &music_file_path) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    };

    match proton_cli::new_sequence(&name_b, &music_file_path) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    };    
}

#[test]
#[should_panic(expected = "Music file not found")]
fn fails_with_nonexistent_music_file_path() {
    let root = common::setup_init_cd();

    let name = "New Sequence".to_string();
    let music_file_path = root.path().join("nonexistent");

    match proton_cli::new_sequence(&name, &music_file_path) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    };
}

#[test]
#[should_panic(expected = "Duplicate music file")]
fn fails_with_existing_destination_file() {
    let root = common::setup_init_cd();

    let name = "New Sequence".to_string();

    let file_name = "TestFile.ogg".to_string();
    let music_file_path = Path::new(&root.path()).join(&file_name);

    // Create file before trying to create sequence
    match File::create(&music_file_path) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    };

    // Create sequence
    match proton_cli::new_sequence(&name, &music_file_path) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    };
}

/// Returns the path to a music file in /.../cli/tests/music/
fn get_music_file_path(file_name: &str) -> PathBuf {
    let mut music_file_path = common::get_test_directory_path();
    music_file_path.push("music");
    music_file_path.push(&file_name);
    
    music_file_path
}

