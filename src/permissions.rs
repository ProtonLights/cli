
use std::path::Path;

use git2::Signature;

use error::Error;
use project_types::{User, Permission, PermissionEnum};
use utils;


pub fn get_permissions() -> Vec<String> {
    vec![
        "Administrate".to_string(),
        "EditSeq".to_string(),
        "EditSeqSec".to_string()
    ]
}

pub fn set_permission(
    auth_user: &User,
    add: bool,
    target_username: &str,
    permission: PermissionEnum,
    target: Option<String>
) -> Result<(), Error> {

    // Only admins (those with GrantPerm permission) can change permissions
    if !auth_user.is_admin() {
        return Err(Error::UnauthorizedAction);
    }

    // Validate and create permission
    let perm = try!(Permission::new(permission, target));

    // Get project that will be modified
    let mut project = try!(utils::read_protonfile(None::<&Path>));

    // Set permissions
    try!(project.set_user_permission(&target_username, perm.to_owned(), add));

    // Save changes to protonfile
    try!(utils::write_protonfile(&project, None::<&Path>));

    // Commit changes
    let signature = Signature::now(&auth_user.name, "proton@teslaworks.net").unwrap();
    let change_type = match add {
        true => "granting",
        false => "revoking",
    };
    let msg = format!("Admin '{}' {} permission '{:?}' to/from user {}",
        auth_user.name, change_type, perm, target_username);

    utils::commit_all(None::<&Path>, &signature, &msg)

}

