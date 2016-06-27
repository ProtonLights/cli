extern crate proton_cli;

mod common;

use std::path::Path;

use common::setup;
use common::rsa_keys::TestKey;
use proton_cli::PermissionEnum;
use proton_cli::utils;


#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_grantperm() {
    let root = setup::setup_init_cd();
    let admin_private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::AdminKeyPem);

    // Create user
    setup::try_new_user(root.path(), "Test User", "a.pub", TestKey::GoodKeyPub);

    // Try to add permission to user
    try_mod_permission(&admin_private_key_path, true, "Test User", PermissionEnum::GrantPerm, None);

    // Now try to remove the permission
    try_mod_permission(&admin_private_key_path, false, "Test User", PermissionEnum::GrantPerm, None);
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_editproj() {
    let root = setup::setup_init_cd();
    let admin_private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::AdminKeyPem);

    // Create user
    setup::try_new_user(root.path(), "Test User", "a.pub", TestKey::GoodKeyPub);

    // Try to add permission to user
    try_mod_permission(&admin_private_key_path, true, "Test User", PermissionEnum::EditProj, None);

    // Now try to remove the permission
    try_mod_permission(&admin_private_key_path, false, "Test User", PermissionEnum::EditProj, None);
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_editseq() {
    let root = setup::setup_init_cd();
    let admin_private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::AdminKeyPem);

    // Create user
    setup::try_new_user(root.path(), "Test User", "a.pub", TestKey::GoodKeyPub);

    // Create sequence
    setup::try_make_sequence("test_seq", "Dissonance.ogg");

    // Try to add permission to user
    try_mod_permission(
        &admin_private_key_path,
        true,
        "Test User",
        PermissionEnum::EditSeq,
        Some("test_seq".to_string()));

    // Now try removing the permission
    try_mod_permission(
        &admin_private_key_path,
        false,
        "Test User",
        PermissionEnum::EditSeq,
        Some("test_seq".to_string()));

}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_editseqsec() {
    let root = setup::setup_init_cd();
    let admin_private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::AdminKeyPem);

    // Create user
    setup::try_new_user(root.path(), "Test User", "a.pub", TestKey::GoodKeyPub);

    // Create sequence
    setup::try_make_sequence("test_seq", "Dissonance.ogg");

    // Try to add permission to user
    try_mod_permission(
        &admin_private_key_path,
        true,
        "Test User",
        PermissionEnum::EditSeqSec,
        Some("test_seq,1".to_string()));

    // Now try removing the permission
    try_mod_permission(
        &admin_private_key_path,
        false,
        "Test User",
        PermissionEnum::EditSeqSec,
        Some("test_seq,1".to_string()));

}

#[test]
#[should_panic(expected = "Invalid permission target")]
fn fails_with_bad_target_editseq() {
    let root = setup::setup_init_cd();
    let admin_private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::AdminKeyPem);

    // Create user
    setup::try_new_user(root.path(), "Test User", "a.pub", TestKey::GoodKeyPub);

    // Create sequence
    setup::try_make_sequence("test_seq", "Dissonance.ogg");

    // Try to add permission to user
    try_mod_permission(
        &admin_private_key_path,
        true,
        "Test User",
        PermissionEnum::EditSeq,
        Some("nonexistent".to_string()));

}

#[test]
#[should_panic(expected = "Invalid permission target")]
fn fails_with_bad_target_editseqsec() {
    let root = setup::setup_init_cd();
    let admin_private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::AdminKeyPem);

    // Create user
    setup::try_new_user(root.path(), "Test User", "a.pub", TestKey::GoodKeyPub);

    // Create sequence
    setup::try_make_sequence("test_seq", "Dissonance.ogg");

    // Try to add permission to user
    try_mod_permission(
        &admin_private_key_path,
        true,
        "Test User",
        PermissionEnum::EditSeqSec,
        Some("section999".to_string()));
}

#[test]
#[should_panic(expected = "Auth user not found")]
fn fails_with_bad_path_to_private_key() {
    let root = setup::setup_init_cd();
    let admin_private_key_path = Path::new("undefined.pem");

    setup::try_new_user(root.path(), "Test User", "a.pub", TestKey::GoodKeyPub);

    try_mod_permission(&admin_private_key_path, true, "Test User", PermissionEnum::EditProj, None);
}

#[test]
fn works_trading_admin_power() {
    let root = setup::setup_init_cd();
    let admin_private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::AdminKeyPem);
    let admin2_private_key_path = common::make_key_file(root.path(), "b.pem", TestKey::GoodKeyPem);

    // Setup new user with GrantPerm permission
    setup::try_new_user(root.path(), "Admin2", "b.pub", TestKey::GoodKeyPub);
    try_mod_permission(&admin_private_key_path, true, "Admin2", PermissionEnum::GrantPerm, None);

    // Now have that new user give the admin another permission
    try_mod_permission(&admin2_private_key_path, false, "admin", PermissionEnum::GrantPerm, None);
}

#[test]
#[should_panic(expected = "Unauthorized action")]
fn fails_modifying_own_permissions() {
    let root = setup::setup_init_cd();
    let admin_private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::AdminKeyPem);
    
    try_mod_permission(&admin_private_key_path, false, "admin", PermissionEnum::GrantPerm, None);
}

#[test]
#[should_panic(expectd = "Auth user not found")]
fn fails_with_unused_private_key() {
    let root = setup::setup_init_cd();
    let admin_private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::GoodKey2Pem);
    
    // Create user
    setup::try_new_user(root.path(), "Test User", "a.pub", TestKey::GoodKeyPub);

    try_mod_permission(&admin_private_key_path, true, "Test User", PermissionEnum::EditProj, None);
}

#[test]
#[should_panic(expected = "User not found")]
fn fails_with_nonexistent_username() {
    let root = setup::setup_init_cd();
    let admin_private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::AdminKeyPem);

    try_mod_permission(&admin_private_key_path, true, "Test User", PermissionEnum::EditProj, None);

}

#[test]
#[should_panic(expected = "Unauthorized action")]
fn fails_with_unauthorized_authority() {
    let root = setup::setup_init_cd();

    // Create user
    setup::try_new_user(root.path(), "Test User", "a.pub", TestKey::GoodKeyPub);
    let private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::GoodKeyPem);

    try_mod_permission(&private_key_path, true, "admin", PermissionEnum::EditProj, None);
}

/// Tries to modify a user's permission
/// Panics on error
///
/// Impure.
fn try_mod_permission<P: AsRef<Path>>(
    auth_private_key_path: P,
    add: bool,
    target_username: &str,
    permission: PermissionEnum,
    target: Option<String>,
) {
    let auth_user = proton_cli::id_user(&auth_private_key_path)
        .expect("Auth user not found");

    match proton_cli::modify_permission(
        &auth_user,
        add,
        &target_username,
        permission.clone(),
        target.clone()
    ) {
        Ok(_) => (),
        Err(e) => panic!("{}", e.to_string()),
    };

    let project = utils::read_protonfile(None::<P>)
        .expect("Error reading project from file");
    let target_user = project.find_user_by_name(&target_username)
        .expect("User target not found")
        .to_owned();

    println!("End permissions: {:?}", target_user.permissions);

    if add {
        assert_eq!(target_user.permissions.len(), 1);
        assert_eq!(target_user.permissions[0].which, permission);
        assert_eq!(target_user.permissions[0].target, target);
    } else {
        assert_eq!(target_user.permissions.len(), 0);
    }
}

