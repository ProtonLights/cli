use rustc_serialize::json;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use dao::ProtonDao;
use error::Error;
use project_types::Project;
use utils;


/// Creates a new Proton project. Returns the public key of the root user.
pub fn new_project<PD: ProtonDao>(
    dao: &PD,
    name: &str,
    layout_id: u32
) -> Result<String, Error> {

    // Check that layout exists
    let _ = try!(dao.get_layout(layout_id));

    // Create keys
    let (root_pub_key, root_private_key) = try!(utils::create_pub_priv_keys());

    // Add project root user
    let root_uid = try!(dao.add_initial_user(name, &root_private_key, &root_pub_key));

    // Give initial user admin permissions
    try!(dao.add_initial_permission(root_uid));

    // Create new project
    let _ = try!(dao.new_project(name, layout_id));

    // Return root user's public key
    Ok(root_pub_key)
}

/// Fetches and returns a project
pub fn get_project<PD: ProtonDao>(
    dao: &PD,
    proj_name: &str
) -> Result<Project, Error> {
    dao.get_project(proj_name)
}

/// Finds and returns a project's layout id
pub fn get_layout_id<PD: ProtonDao>(
    dao: &PD,
    proj_name: &str
) -> Result<u32, Error> {

    // Check that project name is valid
    if !Project::validate_name(proj_name) {
        return Err(Error::InvalidProjectName(proj_name.to_owned()));
    }
    
    // Check that project exists
    let project = try!(dao.get_project(proj_name));

    // Return layout id
    Ok(project.layout_id)
}

/// Gets all sequence data in the project's playlist and writes to file.
/// Returns the path to the data file
pub fn get_playlist_data<PD: ProtonDao> (
    dao: &PD,
    proj_name: &str
) -> Result<PathBuf, Error> {

    // Check that project exists
    let project = try!(dao.get_project(proj_name));

    // Get file ready for writing
    let mut file_path = env::current_dir().expect("Couldn't get current directory");
    file_path.push("tmp_get_playlist_data.json");

    let mut data_file = try!(File::create(&file_path).map_err(Error::Io));

    let mut playlist_data = Vec::with_capacity(project.playlist.len());

    // Go through each sequence in the playlist
    for seqid in project.playlist.iter() {

        print!("Getting sequence data...");

        let seq_data = dao.get_sequence_data(proj_name, seqid.to_owned())?;

        playlist_data.push(seq_data);

        println!("done");
    }

    print!("Encoding playlist data..");
    let json_data = try!(json::encode(&playlist_data).map_err(Error::JsonEncode));
    println!("done");

    print!("Writing data to file...");
    try!(data_file.write_all(json_data.as_bytes()).map_err(Error::Io));
    println!("done");

    Ok(file_path)
}
