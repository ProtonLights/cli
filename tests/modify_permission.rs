extern crate proton_cli;

mod common;

use common::setup;
use common::rsa_keys::TestKey;


#[test]
fn works_with_valid_args() {
    let root = setup::setup_init_cd();

    // Make key file for user
    let key_path_a = common::make_key_file(root.path(), "a.pub", TestKey::GoodKeyPub);
    let admin_user_name = "Admin";

    // Add admin to project
    let _ = proton_cli::new_user(&key_path_a.as_path(), &admin_user_name)
        .expect("Error adding admin user");


}

#[test]
fn fails_with_nonexistent_private_key() {
    
}

#[test]
fn fails_with_nonexistent_user_name() {

}

#[test]
fn fails_with_nonexistent_project_target() {

}

#[test]
fn fails_with_unauthorized_authority() {

}



// allow adds permission
// deny removes permission

