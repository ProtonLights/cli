//! This module manages project users

use std::path::Path;

use Error;
use utils;


/// Creates a new user for the project in the current directory.
/// Assumes the current directory contains a Protonfile.json file.
///
/// 1. Read the new user's public key from the file path given
/// 2. Add the user's name and public key to the protonfile
///
/// Impure.
pub fn new_user<P: AsRef<Path>>(public_key_path: P, name: String) -> Result<(), Error> {

    // Get public key from file
    let pub_key = try!(utils::file_as_string(public_key_path));

    // Load Project from existing file
    let project = try!(utils::read_protonfile());

    // Add user
    let new_project = project.add_user(name, pub_key);

    // Save updated project
    utils::write_protonfile(&new_project)    

}

