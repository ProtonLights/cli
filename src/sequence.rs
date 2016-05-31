//! This module manages project sequences
use std::path::{Path, PathBuf};
use std::fs;

use git2::Signature;

use Error;
use utils;

/// Creates a new user for the project in the current directory.
/// Assumes the current directory contains a Protonfile.json file.
///
/// Impure.
pub fn new_sequence<P: AsRef<Path>>(name: &str, music_file_path: P) -> Result<(), Error> {
    // Get name of music file from path
    let music_file_name = try!(utils::file_name_from_path(&music_file_path));

    // Try to copy music file into current directory
    let dest_path = try!(copy_music_file(&music_file_path));

    // Add sequence to project
    let project = try!(utils::read_protonfile(None::<P>));
    let new_project = match project.add_sequence(name, &music_file_name) {
        Ok(proj) => proj,
        Err(e) => {
            // Remove copied music file (clean up)
            try!(fs::remove_file(&dest_path).map_err(Error::Io));
            panic!(e.to_string())
        },
    };
    try!(utils::write_protonfile(&new_project, None::<P>));

    // Commit changes
    let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();
    let msg = format!("Adding new sequence '{}'", name);
    let pf_path = Path::new("Protonfile.json");
    let repo_path: Option<P> = None;

    utils::commit_file(&pf_path, repo_path, &signature, &msg)
        .map(|_| ())
}

/// Copies the file at music_file_path to the current directory
/// Throw error if file does not exist
/// or file with the same name exists in the current directory
///
/// Impure.
fn copy_music_file<P: AsRef<Path>>(music_file_path: P) -> Result<PathBuf, Error> {
    // Make sure source file exists
    if !music_file_path.as_ref().exists() {
        Err(music_file_not_found_error(music_file_path))
    } else {
        // Make sure destination file doesn't already exist
        let file_name = try!(utils::file_name_from_path(&music_file_path));
        let dest_path = Path::new(&file_name);
        if dest_path.exists() {
            Err(duplicate_music_file_error(dest_path))
        } else {
            try!(fs::copy(&music_file_path, &dest_path)
                .map_err(Error::Io)
                .map(|_| Ok(PathBuf::from(dest_path))))
        }

    }

}

fn music_file_not_found_error<P: AsRef<Path>>(path: P) -> Error {
    let path_as_str = path.as_ref().to_str().expect("Path not valid UTF-8");
    Error::MusicFileNotFound(path_as_str.to_string())
}

fn duplicate_music_file_error<P: AsRef<Path>>(path: P) -> Error {
    match utils::file_name_from_path(path) {
        Ok(file_name) => Error::DuplicateMusicFile(file_name),
        Err(e) => e,
    }
}
