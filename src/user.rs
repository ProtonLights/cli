use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use rustc_serialize::json;

use project_types::Project;
use Error;


pub fn new_user<P: AsRef<Path>>(project: String, public: P, name: String) -> Result<(), Error> {

	// Get public key from file
	let pub_key = try!(file_as_string(public));

	// Get existing Protonfile
    let mut path = PathBuf::from(Path::new(&project));
    path.push("Protonfile.json");
    let protonfile = try!(file_as_string(&path));

    // Load Project from existing file
   	let mut project: Project = try!(json::decode(&protonfile).map_err(Error::JsonDecode));

   	// Add user
   	project.add_user(name, pub_key);

   	// Save file
   	let pretty_json = json::as_pretty_json(&project);
   	File::create(&path)
   		.and_then(|mut protonfile| write!(protonfile, "{}\n", pretty_json))
   		.map_err(Error::Io)
   		.and_then(|_| Ok(()))

}

// Reads a file as a string
fn file_as_string<P: AsRef<Path>>(path: P) -> Result<String, Error> {
	File::open(path)
		.map_err(Error::Io)
		.and_then(|mut file| {
			let mut string = String::new();
			file.read_to_string(&mut string)
				.map_err(Error::Io)
				.and_then(|_| Ok(string))			
		})
}
