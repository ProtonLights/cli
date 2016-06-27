/// Executable for proton_cli
extern crate rustc_serialize;
extern crate proton_cli;
extern crate docopt;

use std::env;
use std::path::Path;
use docopt::Docopt;

use proton_cli::error::Error;
use proton_cli::PermissionEnum;
use proton_cli::utils;


const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  ./proton init <folder> <public-key>
  ./proton new-user <name> <public-key>
  ./proton new-sequence <name> <music-file>
  ./proton id-user <private-key>
  ./proton list-permissions
  ./proton mod-permission <private-key> (add | remove) <name> <permission> [target]
  ./proton (-h | --help)

Options:
  -h --help     Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
	arg_folder: Option<String>,
	arg_public_key: Option<String>,
	arg_private_key: Option<String>,
	arg_name: Option<String>,
	arg_music_file: Option<String>,
	arg_permission: Option<PermissionEnum>,
	arg_target: Option<String>,
}

fn main() {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.decode())
		.unwrap_or_else(|e| e.exit());

	// Below unwrap()'s are safe within Docopt's usage rules

	let command: fn(Args) -> Result<(), Error> = match env::args().nth(1).unwrap().as_ref() {
		"init" => run_init,
		"new-user" => run_new_user,
		"id-user" => run_id_user,
		"new-sequence" => run_new_sequence,
		"list-permissions" => run_list_permissions,
		"mod-permission" => run_modify_permission,
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

	let admin_pub_key_path = args.arg_public_key.unwrap();
	let admin_pub_key = try!(utils::file_as_string(&admin_pub_key_path));
	
	proton_cli::initialize_project(&root_path, &admin_pub_key)
}

fn run_new_user(args: Args) -> Result<(), Error> {
	let public_key = args.arg_public_key.unwrap();
	let public_key_path = Path::new(&public_key);
	let name = args.arg_name.unwrap();
	proton_cli::new_user(&public_key_path, &name)
}

fn run_id_user(args: Args) -> Result<(), Error> {
	let private_key = args.arg_private_key.unwrap();
	try!(proton_cli::id_user(&private_key)
		.map(|user| {
			println!("{:?}", user);
			Ok(())
		})
	)
}

fn run_new_sequence(args: Args) -> Result<(), Error> {
	let name = args.arg_name.unwrap();
	let music_file = args.arg_music_file.unwrap();
	let music_file_path = Path::new(&music_file);
	proton_cli::new_sequence(&name, &music_file_path)
}

#[allow(unused_variables)]
fn run_list_permissions(args: Args) -> Result<(), Error> {
	let perm_as_str = proton_cli::permissions_as_string();
	let permissions = perm_as_str.replace(",", "\n");
	println!("{}", permissions);
	Ok(())
}

fn run_modify_permission(args: Args) -> Result<(), Error> {
	let private_key = args.arg_private_key.unwrap();
	let auth_user = try!(proton_cli::id_user(&private_key));

	let added = env::args().nth(3).unwrap() == "add";
	let username = args.arg_name.unwrap();
	let permission = args.arg_permission.unwrap();
	let target = args.arg_target;

	proton_cli::modify_permission(&auth_user, added, &username, permission, target)
}



