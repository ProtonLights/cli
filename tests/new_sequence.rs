extern crate proton_cli;
extern crate tempdir;
extern crate git2;

mod common;

use std::path::Path;
use std::fs::{self, File, OpenOptions};


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

    let file_name = "TestFile.mp3".to_string();
    let music_file_path = Path::new(&file_name);

    // Create file before trying to create sequence
    match File::create(music_file_path) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    };

    // Create sequence
    match proton_cli::new_sequence(&name, &music_file_path) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    };
}
