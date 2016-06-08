//! This module manages project sequences

use std::path::{Path, PathBuf};
use std::fs;

use git2::Signature;
use sfml::audio::Music;
use regex::Regex;

use Error;
use utils;

/// Creates a new user for the project in the current directory.
/// Assumes the current directory contains a Protonfile.json file.
///
/// Impure.
pub fn new_sequence<P: AsRef<Path>>(name: &str, music_file_path: P) -> Result<(), Error> {
    
    // Make sure the name is valid (needed since it will be used in a file path)
    try!(validate_seq_name(name));

    // Make sure the music file is a valid format
    try!(validate_file_type(&music_file_path));

    // Make the name of the sequence's directory
    let mut sequence_dir = String::from("seq_");
    sequence_dir.push_str(&name);
    let sequence_dir = sequence_dir;

    // Try to create the sequence's directory
    // This also throws an error if the directory already exists and is not empty
    try!(utils::create_empty_directory(Path::new(&sequence_dir))
        .map_err(|_| Error::DuplicateSequence(name.to_string()) ));

    // Get name of music file from path
    let music_file_name = try!(utils::file_name_from_path(&music_file_path));

    // Try to copy music file into sequence directory
    let dest_path = try!(copy_music_file(&music_file_path, &sequence_dir));

    // Get duration of music file
    let music_duration_sec = try!(get_music_duration_sec(&dest_path));

    // Add sequence to project
    let project = try!(utils::read_protonfile(None::<P>));
    let new_project = match project.add_sequence(
        name,
        &sequence_dir,
        &music_file_name,
        music_duration_sec
    ) {
        Ok(proj) => proj,
        Err(e) => {
            // Remove copied music file (clean up)
            try!(fs::remove_file(&dest_path).map_err(Error::Io));
            panic!(e.to_string())
        },
    };

    // Save project
    try!(utils::write_protonfile(&new_project, None::<P>));

    // Commit changes
    let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();
    let msg = format!("Adding new sequence '{}'", name);
    let repo_path: Option<P> = None;

    utils::commit_all(repo_path, &signature, &msg)
        .map(|_| ())
}

/// Check that the music file is a valid format
/// Full list of supported formats can be found at
/// http://www.rust-sfml.org/doc/rsfml/audio/struct.Music.html
fn validate_file_type<P: AsRef<Path>>(music_file_path: P) -> Result<(), Error> {
    match music_file_path.as_ref().extension() {
        Some(extension) => {
            match extension.to_str() {
                Some("ogg")  |
                Some("wav")  |
                Some("flac") |
                Some("aiff") |
                Some("raw") => Ok(()),
                None => Err(unsupported_file_type_error("Extension is not valid unicode")),
                Some(ext) => Err(unsupported_file_type_error(ext)),
            }
        },
        None => Err(unsupported_file_type_error("unknown")),
    }
}

/// Makes sure the name has only valid characters in it
/// A valid character is upper and lower alpha, numbers, and underscores
fn validate_seq_name(name: &str) -> Result<(), Error> {

    let seq_name_regex = Regex::new("^[0-9A-Za-z_]+$").expect("Invalid regex given");
    if seq_name_regex.is_match(name) {
        Ok(())
    } else {
        Err(invalid_sequence_name_error(name))
    }
}

/// Copies the file at music_file_path to the current directory
/// Throw error if file does not exist
///
/// Impure.
fn copy_music_file<P: AsRef<Path>>(music_file_path: P, dest_folder: &str) -> Result<PathBuf, Error> {
    // Make sure source file exists
    if !music_file_path.as_ref().exists() {
        Err(music_file_not_found_error(music_file_path))
    } else {
        let file_name = try!(utils::file_name_from_path(&music_file_path));
        let dest_path = Path::new(&dest_folder).join(&file_name);
        fs::copy(&music_file_path, &dest_path)
            .map_err(Error::Io)
            .map(|_| PathBuf::from(dest_path))
    }

}

/// Extracts the duration of a music file
fn get_music_duration_sec<P: AsRef<Path>>(path: P) -> Result<u32, Error> {
    let path_str = &path.as_ref().to_str().expect("Path is invalid");
    let music = match Music::new_from_file(&path_str) {
        Some(m) => m,
        None => return Err(rsfml_error("Error reading file.")),
    };
    let duration_time = music.get_duration();
    let duration = duration_time.as_seconds() as u32;
    Ok(duration)
}

fn invalid_sequence_name_error(seq_name: &str) -> Error {
    Error::InvalidSequenceName(seq_name.to_string())
}

fn music_file_not_found_error<P: AsRef<Path>>(path: P) -> Error {
    let path_as_str = path.as_ref().to_str().expect("Path not valid UTF-8");
    Error::MusicFileNotFound(path_as_str.to_string())
}

fn rsfml_error(error: &str) -> Error {
    Error::Rsfml(error.to_string())
}

fn unsupported_file_type_error(file_type: &str) -> Error {
    Error::UnsupportedFileType(file_type.to_string())
}