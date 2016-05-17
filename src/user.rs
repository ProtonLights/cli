use std::fs::File;
use std::io::Read;
use std::path::Path;

use Error;


pub fn user_new<P: AsRef<Path>>(public: P, name: String) -> Result<(), Error> {

	File::open(public)
		.map_err(Error::Io)
		.and_then(|mut file| {
			let mut pub_key = String::new();
			file.read_to_string(&mut pub_key)
				.map_err(Error::Io)
				.map(|_| pub_key)
		})
		.and_then(|key| {
			println!("{}\n{}", key, name);
			Ok(())
		})
}
