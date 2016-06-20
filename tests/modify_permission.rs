extern crate proton_cli;

mod common;

use std::path::Path;

use common::setup;
use proton_cli::Permission;
use proton_cli::utils;


#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_adding_with_valid_args() {
    let root = setup::setup_init_cd();

    try_mod_permission("admin", true, "Test User", &Permission::EditProj);
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_removing_with_valid_args() {
    let root = setup::setup_init_cd();

    try_mod_permission("admin", false, "Test User", &Permission::EditProj);
}

#[test]
#[should_panic(expect = "IO Error")]
fn fails_with_bad_path_to_private_key() {
    
}

#[test]
#[should_panic(expect = "User not found")]
fn fails_with_unused_private_key() {
    
}

#[test]
#[should_panic(expect = "User not found")]
fn fails_with_nonexistent_user() {

}

#[test]
#[should_panic(expect = "Unauthorized action")]
fn fails_with_unauthorized_authority() {

}

/// Tries to modify a user's permission
/// Panics on error
///
/// Impure.
fn try_mod_permission(auth_username: &str, add: bool, target_username: &str, permission: &Permission) {
    let project = utils::read_protonfile(None::<&Path>)
        .expect("Error reading project");
    let target_user = project.find_user_by_name(&target_username)
        .expect("User not found");
    let auth_user = project.find_user_by_name(&auth_username)
        .expect("User not found");

    match proton_cli::modify_permission(&auth_user, add, &target_user, permission, &project) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    };

    if add {
        assert_eq!(target_user.permissions.len(), 1);
        assert_eq!(target_user.permissions[0], *permission);
    } else {
        assert_eq!(target_user.permissions.len(), 0);
    }
}

