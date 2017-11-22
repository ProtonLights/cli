use rustc_serialize::json;

use dao::ProtonDao;
use error::Error;
use project_types::{Project, SequenceData};
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

/// Gets all sequence data in the project's playlist
pub fn get_playlist_data<PD: ProtonDao> (
    dao: &PD,
    proj_name: &str
) -> Result<String, Error> {

    // Check that project exists
    let project = try!(dao.get_project(proj_name));

    let mut playlist_data = Vec::with_capacity(project.playlist.len());

    // Go through each sequence in the playlist
    for seqid in project.playlist.iter() {

        print!("Getting sequence {}...", seqid);

        // Get sequence
        let sequence = try!(dao.get_sequence(seqid.to_owned()));

        println!("Sequence '{}' retrieved", &sequence.name);
        print!("Getting channel ids...");

        // Get the sequence's channel ids
        let chan_ids = try!(dao.get_channel_ids(seqid.to_owned()));

        if chan_ids.len() < 1 {
            // TODO: make error
            println!("No channels found");
            panic!("No channels found");
        }

        println!("Channel ids loaded.");
        print!("Getting data...");

        // Create vector for sequence data
        // Up to 512 channels per universe, plus one because DMX starts at 1
        let mut seq_data = vec![vec![0; sequence.num_frames as usize]; 513];

        // Get each channel's data and put it in the correct vector slot
        for chanid in chan_ids {
            let channel = try!(dao.get_channel(chanid));
            let chan_data = try!(dao.get_data(seqid.to_owned(), chanid.to_owned()));
            seq_data[channel.channel_dmx as usize] = chan_data;
        }

        let sequence_data = SequenceData {
            name: sequence.name,
            frame_dur_ms: sequence.frame_duration_ms,
            music_file: sequence.music_file_name,
            num_frames: sequence.num_frames,
            data: seq_data
        };

        playlist_data.push(sequence_data);

        println!("done");
    }

    print!("Encoding playlist data..");
    json::encode(&playlist_data).map_err(Error::JsonEncode)
}
