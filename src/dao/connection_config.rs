use rustc_serialize::json;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use error::Error;

#[derive(Debug, RustcDecodable)]
pub struct ConnectionConfig {
    pub host: String,
    pub password: String
}

impl ConnectionConfig {

    // Loads config from file. If not found, creates new config at specified path
    pub fn load() -> Result<ConnectionConfig, Error> {
        // Config file is in $HOME/.proton_conn.cfg
        let home_dir = env::home_dir().unwrap();
        let config_path = format!("{}/.proton_conn.cfg", home_dir.display());

        // Try to read config file. NEVER version control this file
        match File::open(&config_path) {
            Ok(mut f) => {
                // Read contents into string
                let mut contents = String::new();
                try!(f.read_to_string(&mut contents).map_err(Error::Io));

                // Decode and return
                let config: ConnectionConfig = json::decode(&contents).expect("Failed to decode config file from JSON");
                Ok(config)
            },
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    // Create config file if DNE
                    let mut template_file = try!(File::create(&config_path)
                        .map_err(Error::Io));
                    try!(template_file.write_all(b"{\n\t\"host\":\"HOST\",\n\t\"password\":\"PASSWORD\"\n}\n")
                        .map_err(Error::Io));

                    println!("Connection file created in {}. Please update with proper host/password!", &config_path);

                    Ok(ConnectionConfig {
                        host: String::from("HOST"),
                        password: String::from("PASSWORD")
                    })
                },
                _ => Err(Error::Io(e))
            }
        }
    }
}
