
use std::path::Path;

use Error;


pub fn duplicate_sequence(name: &str) -> Error {
    Error::DuplicateSequence(name.to_string())
}

pub fn duplicate_user(pub_key: &str, name: &str) -> Error {
        Error::DuplicateUser(pub_key.to_string(), name.to_string())
}

pub fn invalid_pub_key(key: &str) -> Error {
    Error::InvalidPublicKey(key.to_string())
}

pub fn invalid_sequence_name(seq_name: &str) -> Error {
    Error::InvalidSequenceName(seq_name.to_string())
}

pub fn music_file_not_found<P: AsRef<Path>>(path: P) -> Error {
    let path_as_str = path.as_ref().to_str().expect("Path not valid UTF-8");
    Error::MusicFileNotFound(path_as_str.to_string())
}

pub fn rsfml(error: &str) -> Error {
    Error::Rsfml(error.to_string())
}

pub fn unsupported_file_type(file_type: &str) -> Error {
    Error::UnsupportedFileType(file_type.to_string())
}