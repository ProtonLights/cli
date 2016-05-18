use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use rustc_serialize::json;

use project_types::Project;
use Error;


pub fn user_new<P: AsRef<Path>>(project: String, public: P, name: String) -> Result<(), Error> {

	// Get public key from file
	let pub_key = file_as_string(public).expect("Error reading public key");

	// Get existing Protonfile
    let mut path = PathBuf::from(Path::new(&project));
    path.push("Protonfile.json");
    let protonfile = file_as_string(&path).expect("Error reading protonfile");

    // Load Project from existing file
   	let mut project: Project = json::decode(&protonfile).expect("Error loading project");

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
