//! This module manages project sequences
use std::path::Path;
use std::fs::File;
use std::io::Cursor;

use git2::Signature;

use Error;
use User;
use utils;


pub fn new_sequence<P: AsRef<Path>>(name: &str, music_file_path: P) -> Result<(), Error> {
    Ok(())
}
