
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
    user: &User,
    project: &Project,
    permission: &Permission,
    allowed: bool
) -> Result<(), Error> {

    Err(Error::TodoErr)
}


