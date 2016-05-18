/// Structure to represent a Proton Project.
/// This is what will be written to a Protonfile at the project root.

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct User {
    pub name: String,
    pub public_key: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Project {
    pub name: String,
    pub users: Vec<User>,
}

impl Project {
    /// Creates an empty project
    pub fn empty() -> Project {
        Project {
            name: "New Project".to_owned(),
            users: Vec::new(),
        }
    }

    // Adds a user to the project
    pub fn add_user(&mut self, name: String, pub_key: String) {
        let user = User {
            name: name,
            public_key: pub_key,
        };

        self.users.push(user);
    }

}

impl PartialEq for Project {
    fn eq(&self, other: &Project) -> bool {
        self.name == other.name
    }
}

impl Eq for Project {
}
