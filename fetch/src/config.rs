use ron::from_str;
use serde::{
    Serialize,
    Deserialize,
};
use std::{
  default::Default,
  error::Error,
  fs::read_to_string,
  path::PathBuf
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub mongo_url: String,
    pub mongo_database: String,
}

impl Config {
    pub fn new(mongo_url: String, mongo_database: String) -> Self {
        Self {
            mongo_url,
            mongo_database,
        }
    }

    pub fn from_file_path(path: PathBuf) -> Result<Self, Box<dyn Error>> {
      let contents = read_to_string(path)?;
      let config = from_str::<Config>(&contents)?;

      Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(
            String::from("mongodb://localhost:27017"),
            String::from("pokrdex")
        )
    }
}
