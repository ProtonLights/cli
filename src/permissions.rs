
use Error;


#[derive(Debug, RustcDecodable)]
pub enum Permission {
    GrantPerm,
    EditProj,
    EditSeq,
    EditSeqSec,
}

pub fn permissions_as_string() -> String {
    String::from("GrantPerm,EditProj,EditSeq,EditSeqSec,")
}

pub fn allow_permission() -> Result<(), Error> {
    Err(Error::TodoErr)
}

pub fn deny_permission() -> Result<(), Error> {
    Err(Error::TodoErr)    
}

