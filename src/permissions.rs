
use error::Error;
use project_types::{User, Project};


#[derive(Clone, Debug, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum PermissionEnum {
    GrantPerm,
    EditProj,
    EditSeq,
    EditSeqSec,
}

#[derive(Clone, Debug, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Permission {
    pub which: PermissionEnum,
    pub target: Option<String>,
}

impl Permission {
    pub fn new(which_enum: PermissionEnum, t: Option<String>) -> Permission {
        Permission {
            which: which_enum,
            target: t,
        }
    }
}

pub fn permissions_as_string() -> String {
    String::from("GrantPerm,EditProj,EditSeq,EditSeqSec,")
}

pub fn modify_permission(
    auth_user: &User,
    add: bool,
    target_user: &User,
    permission: &PermissionEnum,
    project: &Project,
    target: Option<String>
) -> Result<(), Error> {

    if add {
        add_permission(&target_user, permission, &project)
    } else {
        remove_permission(&target_user, permission, &project)
    }
}

#[allow(unused_variables)]
fn add_permission(
    user: &User,
    permission: &PermissionEnum,
    project: &Project,
) -> Result<(), Error> {

    Err(Error::TodoErr)
}

#[allow(unused_variables)]
fn remove_permission(
    user: &User,
    permission: &PermissionEnum,
    project: &Project,
) -> Result<(), Error> {

    Err(Error::TodoErr)
}

