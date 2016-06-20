
use error::Error;
use project_types::User;


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
            &PermissionEnum::GrantPerm => {
                target == &None::<String>
            },
            &PermissionEnum::EditProj => {
                target == &None::<String>
            },
            &PermissionEnum::EditSeq => {
                false
            },
            &PermissionEnum::EditSeqSec => {
                false
            },
        };

        if valid {
            Ok(())
        } else {
            Err(Error::InvalidPermissionTarget)
        }
    }

}

pub fn permissions_as_string() -> String {
    String::from("GrantPerm,EditProj,EditSeq,EditSeqSec,")
}

pub fn modify_permission(
    auth_user: &User,
    add: bool,
    target_user: &mut User,
    permission: PermissionEnum,
    target: Option<String>
) -> Result<(), Error> {

    if !auth_user.is_admin() {
        return Err(Error::UnauthorizedAction);
    }

    let perm = try!(Permission::new(permission, target));

    if add {
        target_user.add_permission(perm)
    } else {
        target_user.remove_permission(perm)
    }

    Ok(())
}

