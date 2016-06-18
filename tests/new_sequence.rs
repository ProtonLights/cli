extern crate proton_cli;
extern crate tempdir;
extern crate git2;

mod common;

use std::path::{Path, PathBuf};

use proton_cli::utils;


#[test]
fn works_with_valid_path_and_name() {
    let root = common::setup_init_cd();
    try_make_sequence("New_Sequence", "Dissonance.ogg");

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
#[should_panic(expected = "Unsupported file type")]
fn fails_with_invalid_file_extension() {
    let root = common::setup_init_cd();
    try_make_sequence("New_Sequence", "Dissonance.mp3");
}

#[test]
#[should_panic(expected = "Duplicate sequence")]
fn fails_with_duplicate_sequence_name() {
    let root = common::setup_init_cd();

    let name = "New_Sequence";

    try_make_sequence(&name, "Dissonance.ogg");

    let music_file_path = get_music_file_path("GlorytotheBells.ogg");

    match proton_cli::new_sequence(&name, &music_file_path) {
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
#[should_panic(expected = "Sequence name had invalid characters")]
fn fails_with_invalid_sequence_name() {
    let root = common::setup_init_cd();
    try_make_sequence("New Sequence", "Dissonance.ogg");
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Music file not found")]
fn fails_with_nonexistent_music_file_path() {
    let root = common::setup_init_cd();
    try_make_sequence("New_Sequence", "nonexistent.ogg");
}

/// Returns the path to a music file in /.../cli/tests/music/
fn get_music_file_path(file_name: &str) -> PathBuf {
    let mut music_file_path = common::get_test_directory_path();
    music_file_path.push("music");
    music_file_path.push(&file_name);
    
    music_file_path
}

/// Attempts to make a new sequence with the given name and music file
/// Panics if error thrown
fn try_make_sequence(name: &str, music_file: &str) {
    let music_file_path = get_music_file_path(music_file);

    let _ = match proton_cli::new_sequence(&name, &music_file_path) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    };
}

