//! This module manages project sequences

use rustc_serialize::json;
use std::path::Path;

use sfml::audio::Music;

use error::Error;
use project_types::{Sequence};
use dao::ProtonDao;
use utils;

/// Creates a new sequence based on proton-vixen-converter data
pub fn new_vixen_sequence<P: AsRef<Path>, PD: ProtonDao>(
    dao: &PD,
    name: &str,
    music_file_path: P,
    seq_duration_ms: u32,
    frame_duration_ms: u32,
    data_file_path: P,
    layout_id: u32
) -> Result<u32, Error> {

    // Get layout (also checks if it exists)
    let layout = try!(dao.get_layout(layout_id));

    // Make sure the music file is a valid format
    try!(validate_file_type(&music_file_path));

    // Get name of music file from path
    let music_file_name = try!(utils::file_name_from_path(&music_file_path));

    // Get duration of music file
    let music_duration_sec = try!(get_music_duration_sec(&music_file_path));
    
    // Create sequence
    let sequence = try!(
        Sequence::new(
            name,
            &music_file_name,
            music_duration_sec,
            seq_duration_ms,
            Some(frame_duration_ms),
            &layout
        )
    );

    // Try to add sequence
    let seq = try!(dao.new_sequence(&sequence));

    // Get sequence channel ids to match up dmx channels with given data
    let chan_ids = try!(dao.get_channel_ids(seq.seqid));

    // Read in vixen sequence data
    let vixen_data_str = try!(utils::file_as_string(data_file_path.as_ref()));
    let vixen_data: Vec<Vec<u16>> = try!(json::decode(&vixen_data_str).map_err(Error::JsonDecode));

    // Make sure the number of channels matches with the layout
    if chan_ids.len() != vixen_data.len() {
        println!("layout: {} vs data: {}", chan_ids.len(), vixen_data.len());
        return Err(Error::InvalidVixenData("Number of channels not the same as the given layout".to_string()));
    }
    
    // For each channel the sequence created, update its data based on vixen_data
    for chanid in chan_ids {
        let channel = try!(dao.get_channel(chanid));
        let ref chan_data = vixen_data[channel.channel_internal as usize - 1]; // TODO, check out of bounds
        try!(dao.new_data(seq.seqid, chanid, chan_data));
    }

    Ok(seq.seqid)
}

/// Creates a new sequence
pub fn new_sequence<P: AsRef<Path>, PD: ProtonDao>(
    dao: &PD,
    name: &str,
    music_file_path: P,
    seq_duration_ms: u32,
    frame_duration_ms: Option<u32>,
    layout_id: Option<u32>
) -> Result<u32, Error> {

    // Get layout (also checks if it exists)
    let lid = match layout_id {
        Some(id) => id,
        None => {
            let default_layout = try!(dao.get_default_layout());
            default_layout.layout_id
        },
    };

    let layout = try!(dao.get_layout(lid));

    // Make sure the music file is a valid format
    try!(validate_file_type(&music_file_path));

    // Get name of music file from path
    let music_file_name = try!(utils::file_name_from_path(&music_file_path));

    // Get duration of music file
    let music_duration_sec = try!(get_music_duration_sec(&music_file_path));
    
    // Create sequence with no data
    let sequence = try!(
        Sequence::new(
            name,
            &music_file_name,
            music_duration_sec,
            seq_duration_ms,
            frame_duration_ms,
            &layout
        )
    );

    // Try to add sequence
    let seq = try!(dao.new_sequence(&sequence));

    // Get list of channel ids in seq, sorted by dmx channel
    let channel_ids = try!(dao.get_channel_ids(seq.seqid));

    // Try to add empty sequence data
    let seq_data = vec![0; sequence.num_frames as usize];
    let _ = try!(dao.new_data_default(seq.seqid, channel_ids, seq_data));

    Ok(seq.seqid)
}

/// Adds a sequence to the project's playlist at the given index
pub fn insert_sequence<PD: ProtonDao> (
    dao: &PD,
    proj_name: &str,
    seqid: u32,
    index: Option<u32>
) -> Result<(), Error> {

    // Check that seqid exists
    let _ = try!(dao.get_sequence(seqid));

    // Get project
    let project = try!(dao.get_project(proj_name));

    // Get offset to insert at (default is end of playlist)
    let offset = index.unwrap_or(project.playlist.len() as u32);

    // Add sequence to project's playlist
    let new_project = try!(project.insert_sequence(seqid, offset));
    dao.update_project(new_project)
}

/// Removes a sequence from a project
pub fn remove_sequence<PD: ProtonDao> (
    dao: &PD,
    proj_name: &str,
    seqid: u32
) -> Result<(), Error> {

    // Remove sequence from project's playlist
    let project = try!(dao.get_project(proj_name));
    let new_project = try!(project.remove_sequence(seqid));
    dao.update_project(new_project)

    // TODO: Remove sequence's music file if not used elsewhere in playlist

}

/// Deletes sequence from storage
pub fn delete_sequence<PD: ProtonDao>(dao: &PD, seq_name: &str) -> Result<(), Error> {
    dao.delete_sequence(seq_name)
}

/// Fetches and returns a sequence
pub fn get_sequence<PD: ProtonDao>(dao: &PD, seqid: u32) -> Result<Sequence, Error> {
    dao.get_sequence(seqid)
}

/// Check that the music file is a valid format
/// Full list of supported formats can be found at
/// http://www.rust-sfml.org/doc/rsfml/audio/struct.Music.html
fn validate_file_type<P: AsRef<Path>>(music_file_path: P) -> Result<(), Error> {
    match music_file_path.as_ref().extension() {
        Some(extension) => {
            match extension.to_str() {
                Some("ogg")  |
                Some("wav")  |
                Some("flac") |
                Some("aiff") |
                Some("raw") => Ok(()),
                None => Err(
                    Error::UnsupportedFileType("Extension is not valid unicode".to_string())
                    ),
                Some(ext) => Err(Error::UnsupportedFileType(ext.to_string())),
            }
        },
        None => Err(Error::UnsupportedFileType("No file extension".to_string())),
    }
}

/// Extracts the duration of a music file
fn get_music_duration_sec<P: AsRef<Path>>(path: P) -> Result<u32, Error> {
    let path_str = &path.as_ref().to_str().expect("Path is invalid");
    let music = match Music::from_file(&path_str) {
        Some(m) => m,
        None => return Err(Error::Rsfml("Error reading file.".to_string())),
    };
    let duration_time = music.duration();
    let duration = duration_time.as_seconds() as u32;
    Ok(duration)
}
