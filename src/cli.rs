extern crate clap;

use self::clap::{App, Arg};

use ::std::env;
use ::std::path::{
    Path,
    PathBuf
};

const NAME: &'static str = env!("CARGO_PKG_NAME");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const ABOUT: &'static str = "
Simple ITH text cleaner.";

#[inline(always)]
///Shortcut to create CLI argument
pub fn arg(name: &str) -> Arg {
    Arg::with_name(name)
}

#[inline(always)]
///Shortcut to create CLI option/flag
pub fn flag(name: &str) -> Arg {
    arg(name).long(name)
}

pub fn parser() -> App<'static, 'static> {
    App::new(NAME).about(ABOUT)
                  .author(AUTHOR)
                  .version(VERSION)
                  .arg(flag("config").short("c")
                                     .takes_value(true)
                                     .help("Path towards configuration file in TOML format. If not specified, '<binary-dir>/vn-text-trim.toml' is used."))
}

///Retrieves configuration of Fie.
fn default_config() -> PathBuf {
    let mut result = env::current_exe().unwrap();

    result.set_extension("toml");

    result
}

#[derive(Debug)]
pub struct Args {
    ///Path to config file
    pub config: PathBuf
}

impl Args {
    pub fn new() -> Self {
        let matches = parser().get_matches();

        let config = matches.value_of("config").map(|config| Path::new(config).to_path_buf())
                                               .unwrap_or(default_config());

        Args {
            config
        }
    }
}
