
use std::path::Path;

use error::Error;
use utils;


#[derive(Clone, Debug, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum PermissionEnum {
    Administrate,
    EditSeq,
    EditSeqSec,
}

#[derive(Clone, Debug, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Permission {
    pub which: PermissionEnum,
    pub target: Option<String>,
}

impl Permission {
    /// Creates a new Permission, joining a permission type with a target
    /// Returns an error if the target is invalid
    pub fn new(which_enum: PermissionEnum, t: Option<String>) -> Result<Permission, Error> {
        // Make sure the target is valid for the given permission type
        try!(Permission::validate_permission(&which_enum, &t));

        // Create permission if valid
        Ok(Permission {
            which: which_enum,
            target: t,
        })
    }

    /// Validates the target for the given permission type
    /// Returns error if invalid target
    fn validate_permission(permission: &PermissionEnum, target: &Option<String>) -> Result<(), Error> {
        
        let valid = match permission {
            &PermissionEnum::Administrate => {
                target == &None::<String>
            },
            &PermissionEnum::EditSeq => {
                if target.is_none() {
                    false
                } else {
                    let seq_name = target.to_owned().unwrap();
                    let project = try!(utils::read_protonfile(None::<&Path>));
                    project.find_sequence_by_name(&seq_name).is_some()
                }
            },
            &PermissionEnum::EditSeqSec => {
                if target.is_none() {
                    false
                } else {
                    let target_str = target.to_owned().unwrap();
                    let targets: Vec<&str> = target_str.split(",").collect();
                    if targets.len() != 2 {
                        false
                    } else {
                        let seq_name = targets[0];
                        let section_num_str = targets[1];
                        let section_num = match section_num_str.parse::<u32>() {
                            Ok(n) => n,
                            Err(_) => return Err(Error::InvalidPermissionTarget), 
                        };
                        let project = try!(utils::read_protonfile(None::<&Path>));
                        match project.find_sequence_by_name(&seq_name) {
                            Some(seq) => {
                                section_num > 0 && section_num <= seq.num_sections
                            },
                            None => false,
                        }

                    }
                }
            },
        };

        if valid {
            Ok(())
        } else {
            Err(Error::InvalidPermissionTarget)
        }
    }
}
