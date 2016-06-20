
use error::Error;
use project_types::{User, Project};


#[derive(Clone, Debug, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum Permission {
    GrantPerm,
    EditProj,
    EditSeq,
    EditSeqSec,
}

pub fn permissions_as_string() -> String {
    String::from("GrantPerm,EditProj,EditSeq,EditSeqSec,")
}

pub fn modify_permission(
    auth_user: &User,
    add: bool,
    target_user: &User,
    permission: &Permission,
    project: &Project,
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
    permission: &Permission,
    project: &Project,
) -> Result<(), Error> {

    Err(Error::TodoErr)
}

#[allow(unused_variables)]
fn remove_permission(
    user: &User,
    permission: &Permission,
    project: &Project,
) -> Result<(), Error> {

    Err(Error::TodoErr)
}

