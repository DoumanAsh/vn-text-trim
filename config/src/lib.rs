//!Configuration module

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate regex;

extern crate utils;

use std::fs::{
    File
};
use std::path::{
    Path
};
use std::io::{
    BufReader,
    Read
};

use utils::ResultExt;

mod re;

fn default_dialogue_re() -> regex::Regex {
    regex::Regex::new("[「（]\\s*([^」 ）]+)").unwrap()
}

#[derive(Deserialize, Debug)]
pub struct Dialogue {
    ///Whether to set extract dialogue or not.
    #[serde(default)]
    pub extract: bool,
    #[serde(default="default_dialogue_re")]
    #[serde(deserialize_with="re::deserialize_to_regex")]
    pub re: regex::Regex,
}

impl Default for Dialogue {
    fn default() -> Self {
        Dialogue {
            extract: false,
            re: default_dialogue_re()
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Replace {
    #[serde(deserialize_with="re::deserialize_to_regex")]
    pub pattern: regex::Regex,
    pub replacement: String,
    #[serde(default)]
    pub limit: usize
}

#[derive(Deserialize, Debug, Default)]
///Configuration of application
pub struct Config {
    #[serde(default)]
    pub dialogue: Dialogue,
    pub replace: Option<Vec<Replace>>
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let file = File::open(path).map_err_to_string("Cannot open config file.")?;
        let mut file = BufReader::new(file);

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).map_err_to_string("Unable to read config file.")?;

        toml::from_str(&buffer).map_err_to_string("Invalid config file.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let result = Config::from_file(Path::new("../vn-text-trim.toml")).unwrap();

        assert!(result.dialogue.extract);
        assert_eq!(result.dialogue.re.as_str(), "[「（]\\s*([^」 ）]+)");
    }
}
