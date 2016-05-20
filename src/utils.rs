use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use rustc_serialize::json;

use Project;
use Error;


/// Reads a Project from a Protonfile.
/// Wraps any errors in proton_cli::Error
/// Assumes Protonfile.json resides in the current directory
pub fn read_protonfile() -> Result<Project, Error> {
    let protonfile_path = Path::new("Protonfile.json");
    let protonfile = try!(file_as_string(&protonfile_path));
    json::decode(&protonfile).map_err(Error::JsonDecode)
}

/// Saves a Project to a Protonfile.
/// Assumes the Protonfile is in the current directory
pub fn write_protonfile(project: &Project) -> Result<(), Error> {
    let pretty_json = json::as_pretty_json(&project);
    let protonfile_path = Path::new("Protonfile.json");
    File::create(&protonfile_path)
        .and_then(|mut protonfile| write!(protonfile, "{}\n", pretty_json))
        .map_err(Error::Io)
}

/// Reads a file as a string.
/// Wraps Read::read_to_string errors in proton_cli::Error
pub fn file_as_string<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    File::open(path)
        .and_then(|mut file| {
            let mut string = String::new();
            file.read_to_string(&mut string)
                .and_then(|_| Ok(string))           
        })
        .map_err(Error::Io)
}
