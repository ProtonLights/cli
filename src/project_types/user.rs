
use Permission;


#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct User {
    pub name: String,
    pub public_key: String,
    pub permissions: Vec<Permission>,
}

impl User {

    pub fn new(name: &str, pub_key: &str) -> User {
        User {
            name: name.to_string(),
            public_key: pub_key.to_string(),
            permissions: Vec::new(),
        }
    }

    pub fn has_permission(&self, perm: &Permission) -> bool {
        false
    }

}
