/// Structure to represent a Proton Project.
/// This is what will be written to a Protonfile at the project root.

#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct User {
    pub name: String,
    pub public_key: String,
}

#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
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
    pub fn add_user(&self, name: String, pub_key: String) -> Project {
        let user = User {
            name: name,
            public_key: pub_key,
        };
        let mut new_project = self.clone();
        new_project.users.push(user);
        new_project
    }

}

impl PartialEq for Project {
    fn eq(&self, other: &Project) -> bool {
        self.name == other.name
    }
}

impl Eq for Project {
}
