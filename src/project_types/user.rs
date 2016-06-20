
use std::io::Cursor;
use openssl::crypto::rsa::RSA as openssl_RSA;

use error::Error;
use Permission;
use PermissionEnum;


#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct User {
    pub name: String,
    pub public_key: String,
    pub permissions: Vec<Permission>,
}

impl User {

    pub fn new(name: &str, pub_key: &str) -> Result<User, Error> {
        try!(User::validate_public_key(&pub_key));

        Ok(User {
            name: name.to_string(),
            public_key: pub_key.to_string(),
            permissions: Vec::new(),
        })
    }

    /// Checks if the given public key is valid
    pub fn validate_public_key(pub_key: &str) -> Result<(), Error> {
        let mut pub_key_readable = Cursor::new(pub_key.to_string());
        try!(openssl_RSA::public_key_from_pem(&mut pub_key_readable)
            .map(|_| Ok(()))
            .map_err(|_| Error::InvalidPublicKey(pub_key.to_string())))
    }

    #[allow(unused_variables)]
    pub fn add_permission(&self, perm: &Permission, target: Option<String>) -> Result<(), Error> {
        Err(Error::TodoErr)
    }

    #[allow(unused_variables)]
    pub fn remove_permission(&self, perm: &Permission, target: Option<String>) -> Result<(), Error> {
        Err(Error::TodoErr)
    }

    pub fn has_permission(&self, perm: &Permission) -> bool {
        
        for p in &self.permissions {
            if p == perm {
                return true;
            }
        }

        false
    }

    pub fn is_admin(&self) -> bool {
        let admin_permission = Permission::new(PermissionEnum::GrantPerm, None::<String>);
        self.has_permission(&admin_permission)
    }

}
