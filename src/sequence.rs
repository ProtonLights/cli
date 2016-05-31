//! This module manages project sequences
use std::path::Path;
use std::fs::{self, File, OpenOptions};

use git2::Signature;

use Error;
use User;
use utils;


pub fn new_sequence<P: AsRef<Path>>(name: &str, music_file_path: P) -> Result<(), Error> {
    // Try to copy music file into current directory
    try!(copy_music_file(music_file_path));

    // Add sequence to project
    let project = try!(utils::read_protonfile(None::<P>));

    Ok(())
}

fn copy_music_file<P: AsRef<Path>>(music_file_path: P) -> Result<(), Error> {
    // Make sure source file exists
    if !music_file_path.as_ref().exists() {
        Err(music_file_not_found_error(music_file_path))
    } else {
        // Make sure destination file doesn't already exist
        let dest_path = Path::new(music_file_path
            .as_ref()
            .file_name()
            .expect("Bad music file path"));
        if dest_path.exists() {
            Err(duplicate_music_file_error(dest_path))
        } else {
            try!(fs::copy(&music_file_path, &dest_path)
                .map_err(Error::Io)
                .map(|_| Ok(())))
        }

    }

}

fn music_file_not_found_error<P: AsRef<Path>>(path: P) -> Error {
    let path_as_str = path.as_ref().to_str().expect("Path not valid UTF-8");
    Error::MusicFileNotFound(path_as_str.to_string())
}

fn duplicate_music_file_error<P: AsRef<Path>>(path: P) -> Error {
    let name_os = path.as_ref().file_name().expect("Bad music file path");
    let name = name_os.to_str().expect("Not a UTF file name");
    Error::DuplicateMusicFile(name.to_string())
}
