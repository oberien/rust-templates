use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use toml::de;
use toml::ser;

use error::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Config> {
        let mut config = String::new();
        {
            let mut config_file = File::open(path)?;
            config_file.read_to_string(&mut config)?;
        }
        let cfg = de::from_str(&config)?;
        Ok(cfg)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
            let contents = ser::to_string(self)?;
            let mut file = OpenOptions::new().create(true).write(true)
                .truncate(true).open(path)?;
            write!(file, "{}", contents)?;
            Ok(())
    }
}
