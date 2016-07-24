extern crate proton_cli;
extern crate tempdir;

mod common;

use std::path::{Path, PathBuf};

use common::rsa_keys::TestKey;
use common::setup;


#[test]
fn works_with_valid_admin_key_and_name() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let name = "UserA";

    setup::try_new_user(
        &root_key_path.as_path(),
        root.path(),
        &name,
        "a.pub",
        TestKey::GoodKeyPub);

    proton_cli::remove_user(&root_key_path, &name).expect("Error removing user");

    // Make sure user was removed
    let project = proton_cli::utils::read_protonfile(None::<&Path>)
        .expect("Error reading project from file");

    assert!(project.find_user_by_name(&name).is_none());
    common::assert_repo_no_modified_files(&root.path()); 
}

#[test]
#[should_panic(expected = "Error removing user: Ssl")]
fn fails_with_bad_admin_key() {

}

#[test]
#[should_panic(expected = "Error removing user: Io")]
fn fails_with_nonexistent_admin_key() {

}

#[test]
#[should_panic(expected = "Error removing user: UnauthorizedAction")]
fn fails_with_unprivileged_admin_key() {

}

#[test]
#[should_panic(expected = "Error removing user: UserNotFound")]
fn fails_with_unknown_name() {

}
