
/// Structure to represent a Proton Project.
/// This is what will be written to a Protonfile at the project root.
#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Project {
    pub name: String,
    pub users: Vec<User>,
}

#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct User {
    pub name: String,
    pub public_key: String,
}

impl Project {
    /// Creates an empty project
    pub fn empty() -> Project {
        Project {
            name: "New Project".to_owned(),
            users: Vec::new(),
        }
    }

    /// Finds a user in the users vector
    /// Returns true if found, else false
    pub fn find_user(&self, user: &User) -> bool {
        let myself = self.clone();
        println!("{:?}", myself);
        for u in myself.users {
            if user == &u {
                return true;
            }
        }
        return false;
    }

    /// Adds a user to the project
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

impl PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        self.name == other.name &&
        self.public_key == other.public_key
    }
}

impl Eq for User {
}
