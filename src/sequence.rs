//! This module manages project sequences
use std::path::Path;
use std::fs;

use Error;
use utils;


pub fn new_sequence<P: AsRef<Path>>(name: &str, music_file_path: P) -> Result<(), Error> {
    // Try to copy music file into current directory
    try!(copy_music_file(&music_file_path));

    // Get name of music file from path
    let music_file_name = try!(file_name_from_path(&music_file_path));

    // Add sequence to project
    let project = try!(utils::read_protonfile(None::<P>));
    let new_project = try!(project.add_sequence(name, &music_file_name));

    Ok(())
}

fn copy_music_file<P: AsRef<Path>>(music_file_path: P) -> Result<(), Error> {
    // Make sure source file exists
    if !music_file_path.as_ref().exists() {
        Err(music_file_not_found_error(music_file_path))
    } else {
        // Make sure destination file doesn't already exist
        let file_name = try!(file_name_from_path(&music_file_path));
        let dest_path = Path::new(&file_name);
        if dest_path.exists() {
            Err(duplicate_music_file_error(dest_path))
        } else {
            try!(fs::copy(&music_file_path, &dest_path)
                .map_err(Error::Io)
                .map(|_| Ok(())))
        }

    }

}

fn file_name_from_path<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    match path.as_ref().file_name() {
        Some(name_os) => {
            match name_os.to_str() {
                Some(name) => Ok(name.to_string()),
                None => Err(Error::InvalidFileName),
            }
        },
        None => Err(Error::InvalidFileName),
    }
}

fn music_file_not_found_error<P: AsRef<Path>>(path: P) -> Error {
    let path_as_str = path.as_ref().to_str().expect("Path not valid UTF-8");
    Error::MusicFileNotFound(path_as_str.to_string())
}

fn duplicate_music_file_error<P: AsRef<Path>>(path: P) -> Error {
    match file_name_from_path(path) {
        Ok(file_name) => Error::DuplicateMusicFile(file_name),
        Err(e) => e,
    }
}
