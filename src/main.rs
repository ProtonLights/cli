/// Executable for proton_cli

extern crate proton_cli;
extern crate git2;
extern crate rustc_serialize;
extern crate docopt;

use std::path::Path;
use docopt::Docopt;
use proton_cli::Error;


const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  ./proton init <folder>
  ./proton user new <public> <name>
  ./proton (-h | --help)

Options:
  -h --help     Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
	cmd_init: bool,
	arg_folder: Option<String>,
	cmd_user: bool,
	cmd_new: bool,
	arg_public: Option<String>,
	arg_name: Option<String>,
}

fn main() {

	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.decode())
		.unwrap_or_else(|e| e.exit());

	let mut result: Result<(), Error> = Err(Error::ArgumentNotFound);

	if args.cmd_init {
		let root = args.arg_folder.expect("Folder cannot be 'None'");
		let root_path = Path::new(&root);
		result = proton_cli::initialize_project(&root_path);
	} else if args.cmd_user {
		if args.cmd_new {
			let public_key = args.arg_public.expect("Public key cannot be 'None'");
			let public_key_path = Path::new(&public_key);
			let name = args.arg_name.expect("Name cannot be 'None'");
			println!("{}, {}", public_key, name);
			result = Ok(());
			//result = proton_cli::user_new(&public_key_path, &name);
		}
	}

	match result {
		Ok(_) => println!("Worked!"),
		Err(e) => println!("{:?}", e.to_string()),
	}

}
