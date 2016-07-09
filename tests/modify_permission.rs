extern crate proton_cli;

mod common;

use std::path::Path;

use common::setup;
use common::rsa_keys::TestKey;
use proton_cli::project_types::PermissionEnum;


#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_grantperm() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    setup::try_new_user(
        &root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    // Try to add permission to user
    try_set_permission(&root_private_key_path, true, "Test User", PermissionEnum::GrantPerm, None);

    // Now try to remove the permission
    try_set_permission(&root_private_key_path, false, "Test User", PermissionEnum::GrantPerm, None);

    // Make sure changes were saved
    common::assert_repo_no_modified_files(&root.path());
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_editproj() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    // Try to add permission to user
    try_set_permission(&root_private_key_path, true, "Test User", PermissionEnum::EditProj, None);

    // Now try to remove the permission
    try_set_permission(&root_private_key_path, false, "Test User", PermissionEnum::EditProj, None);

    // Make sure changes were saved
    common::assert_repo_no_modified_files(&root.path());
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_editseq() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    // Create sequence
    setup::try_make_sequence(&root_private_key_path, "test_seq", "Dissonance.ogg");

    // Try to add permission to user
    try_set_permission(
        &root_private_key_path,
        true,
        "Test User",
        PermissionEnum::EditSeq,
        Some("test_seq".to_string()));

    // Now try removing the permission
    try_set_permission(
        &root_private_key_path,
        false,
        "Test User",
        PermissionEnum::EditSeq,
        Some("test_seq".to_string()));

    // Make sure changes were saved
    common::assert_repo_no_modified_files(&root.path());
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_editseqsec() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    // Create sequence
    setup::try_make_sequence(&root_private_key_path.as_path(), "test_seq", "Dissonance.ogg");

    // Try to add permission to user
    try_set_permission(
        &root_private_key_path,
        true,
        "Test User",
        PermissionEnum::EditSeqSec,
        Some("test_seq,1".to_string()));

    // Now try removing the permission
    try_set_permission(
        &root_private_key_path,
        false,
        "Test User",
        PermissionEnum::EditSeqSec,
        Some("test_seq,1".to_string()));

    // Make sure changes were saved
    common::assert_repo_no_modified_files(&root.path());

}

#[test]
#[should_panic(expected = "Invalid permission target")]
fn fails_with_bad_target_editseq() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    // Create sequence
    setup::try_make_sequence(&root_private_key_path, "test_seq", "Dissonance.ogg");

    // Try to add permission to user
    try_set_permission(
        &root_private_key_path,
        true,
        "Test User",
        PermissionEnum::EditSeq,
        Some("nonexistent".to_string()));

}

#[test]
#[should_panic(expected = "Invalid permission target")]
fn fails_with_bad_target_editseqsec() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    // Create sequence
    setup::try_make_sequence(&root_private_key_path, "test_seq", "Dissonance.ogg");

    // Try to add permission to user
    try_set_permission(
        &root_private_key_path,
        true,
        "Test User",
        PermissionEnum::EditSeqSec,
        Some("section999".to_string()));
}

#[test]
#[should_panic(expected = "entity not found")]
fn fails_with_bad_path_to_private_key() {
    let root = setup::setup_init_cd();
    let root_private_key_path = Path::new("undefined.pem");

    setup::try_new_user(
        root_private_key_path,
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    try_set_permission(&root_private_key_path, true, "Test User", PermissionEnum::EditProj, None);
}

#[test]
fn works_trading_admin_power() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);
    let admin2_private_key_path = common::make_key_file(root.path(), "b.pem", TestKey::GoodKeyPem);

    // Setup new user with GrantPerm permission
    setup::try_new_user(
        &root_private_key_path.as_path(),
        root.path(),
        "Admin2",
        "b.pub",
        TestKey::GoodKeyPub);
    try_set_permission(&root_private_key_path, true, "Admin2", PermissionEnum::GrantPerm, None);

    // Now have that new user take away the first's GrantPerm permission
    try_set_permission(&admin2_private_key_path, false, "root", PermissionEnum::GrantPerm, None);

    // Make sure changes were saved
    common::assert_repo_no_modified_files(&root.path());
}

#[test]
#[should_panic(expected = "Unauthorized action")]
fn fails_modifying_own_permissions() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);
    
    try_set_permission(&root_private_key_path, false, "root", PermissionEnum::GrantPerm, None);
}

#[test]
#[should_panic(expectd = "Auth user not found")]
fn fails_with_unused_private_key() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::GoodKey2Pem);
    
    // Create user
    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    try_set_permission(&root_private_key_path, true, "Test User", PermissionEnum::EditProj, None);
}

#[test]
#[should_panic(expected = "User not found")]
fn fails_with_nonexistent_username() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    try_set_permission(&root_private_key_path, true, "Test User", PermissionEnum::EditProj, None);

}

#[test]
#[should_panic(expected = "Unauthorized action")]
fn fails_with_unauthorized_authority() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);
    let private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::GoodKeyPem);

    try_set_permission(&private_key_path, true, "root", PermissionEnum::EditProj, None);
}

/// Tries to modify a user's permission
/// Panics on error
///
/// Impure.
fn try_set_permission<P: AsRef<Path>>(
    auth_private_key_path: P,
    add: bool,
    target_username: &str,
    permission: PermissionEnum,
    target: Option<String>,
) {
    let auth_user = proton_cli::id_user(&auth_private_key_path)
        .expect("Auth user not found");

    match proton_cli::set_permission(
        &auth_user,
        add,
        &target_username,
        permission.clone(),
        target.clone()
    ) {
        Ok(_) => (),
        Err(e) => panic!("{}", e.to_string()),
    };

}

