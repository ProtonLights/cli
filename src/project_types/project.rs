
use project_types::{Sequence, User};
use error::Error;
use Permission;


/// Structure to represent a Proton Project.
/// This is what will be written to a Protonfile at the project root.
#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Project {
    pub name: String,
    pub users: Vec<User>,
    pub sequences: Vec<Sequence>,
}

impl Project {
    /// Creates an empty project with the given admin user
    pub fn empty(admin_pub_key: &str) -> Result<Project, Error> {

        let mut admin = try!(User::new("admin".as_ref(), &admin_pub_key));
        admin.permissions.push(Permission::GrantPerm);
        Ok(Project {
            name: "New Project".to_owned(),
            users: vec![admin],
            sequences: Vec::new(),
        })
    }

    /// Finds a user with the given public key
    /// Returns the user if found, else None
    fn find_user_by_public_key(&self, pub_key: &str) -> Option<&User> {
        for u in &self.users {
            if u.public_key == pub_key {
                return Some(u);
            }
        }
        None::<&User>
    }

    /// Finds a user with the given name
    /// Returns the user if found, else None
    pub fn find_user_by_name(&self, name: &str) -> Option<&User> {
        for u in &self.users {
            if u.name == name {
                return Some(u);
            }
        }
        None::<&User>
    }

    /// Finds a user in the users vector
    /// Returns true if found, else false
    pub fn user_exists(&self, user: &User) -> bool {
        for u in &self.users {
            if user == u {
                return true;
            }
        }
        return false;
    }

    /// Adds a user to the project
    /// Returns a new project with the user added
    pub fn add_user(&self, name: &str, pub_key: &str) -> Result<Project, Error> {
        
        let user = try!(User::new(name, pub_key));

        if self.find_user_by_name(name).is_some() ||
           self.find_user_by_public_key(pub_key).is_some() {
           
            Err(Error::DuplicateUser(pub_key.to_string(), name.to_string()))
        } else {
            let mut new_project = self.clone();
            new_project.users.push(user);
            Ok(new_project)
        }
    }

    /// Adds a sequence to the project
    /// Returns a new project with the sequence added
    pub fn add_sequence(
        &self,
        name: &str,
        directory_name: &str,
        music_file_name: &str,
        music_duration_sec: u32,
    ) -> Result<Project, Error> {
    
        let sequence = try!(Sequence::new(
            name,
            directory_name,
            music_file_name,
            music_duration_sec,
            None,
            None,
        ));

        // Check if duplicate
        let mut exists = false;
        for s in &self.sequences {
            if s.name == name
            || s.directory_name == directory_name {
                exists = true;
                break;
            }
        }

        if exists {
            Err(Error::DuplicateSequence(name.to_string()))
        } else {
            let mut new_project = self.clone();
            new_project.sequences.push(sequence);
            Ok(new_project)
        }
    }
}

