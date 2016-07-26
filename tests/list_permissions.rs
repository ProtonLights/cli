extern crate proton_cli;

mod common;

use std::path::Path;

use common::setup;
use common::rsa_keys::TestKey;
use proton_cli::project_types::PermissionEnum;


#[test]
fn works_with_valid_key_no_permissions() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let name = "UserA";

    setup::try_new_user(
        &root_key_path.as_path(),
        root.path(),
        &name,
        "a.pub",
        TestKey::GoodKeyPub);

    
}

#[test]
fn works_with_valid_key_one_permission() {

}

#[test]
fn works_with_valid_key_all_permissions() {

}

#[test]
#[should_panic(expected = "Error listing permissions: Io")]
fn fails_with_invalid_key_path() {

}

#[test]
#[should_panic(expected = "Error listing permissions: Ssl")]
fn fails_with_invalid_key() {

}
