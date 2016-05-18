/// Executable for proton_cli

extern crate proton_cli;
extern crate git2;
extern crate rustc_serialize;
extern crate docopt;

use std::env;
use std::path::Path;
use docopt::Docopt;
use proton_cli::Error;


const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  ./proton init <folder>
  ./proton new-user <folder> <public-key> <name>
  ./proton (-h | --help)

Options:
  -h --help     Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
	cmd_init: bool,
	cmd_new_user: bool,
	arg_folder: Option<String>,
	arg_public_key: Option<String>,
	arg_name: Option<String>,
}

fn main2() {
	let root_arg = env::args().nth(2).unwrap();
	let root = Path::new(&root_arg);
	match proton_cli::initialize_project(root) {
		Ok(_) => println!("Worked!"),
		Err(e) => println!("{:?}", e),
	}
}

fn main() {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.decode())
		.unwrap_or_else(|e| e.exit());

	// Below unwrap()'s are safe within Docopt's usage rules

	let command: fn(Args) -> Result<(), Error> = match env::args().nth(1).unwrap().as_ref() {
		"init" => run_init,
		"new-user" => run_new_user,
		_ => panic!("Invalid first argument"),
	};

	let result = command(args);
	match result {
		Ok(_) => println!("Worked!"),
		Err(e) => println!("{:?}", e.to_string()),
	};

}

fn run_init(args: Args) -> Result<(), Error> {
	let root = args.arg_folder.unwrap();
	let root_path = Path::new(&root);
	proton_cli::initialize_project(&root_path)
}

fn run_new_user(args: Args) -> Result<(), Error> {
	let folder = args.arg_folder.unwrap();
	let public_key = args.arg_public_key.unwrap();
	let public_key_path = Path::new(&public_key);
	let name = args.arg_name.unwrap();
	println!("{}, {}, {}", folder, public_key, name);
	Ok(())
	//proton_cli::add_user(folder, public_key_path, name)
}
