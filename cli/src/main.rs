#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate argparse;

mod error;
mod logger;
mod config;

use error::*;
use config::Config;
use argparse::{ArgumentParser, Store};

fn main() {
    logger::init().unwrap();
    let mut config_file = "Config.toml".to_string();
    {  // this block limits scope of borrows by ap.refer() method
        debug!("Parsing Arguments");
        let mut ap = ArgumentParser::new();
        ap.set_description("{{name}}");
        ap.refer(&mut config_file)
            .add_option(&["--config"], Store, "Config file to use");
        ap.parse_args_or_exit();
        debug!("Arguments successfully parsed");
    }

    debug!("Loading config file");
    let cfg = Config::load(&config_file).unwrap();
    debug!("Got config: {:?}", cfg);
}
