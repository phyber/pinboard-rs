extern crate config;
extern crate pinboard;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod settings;

use std::io::{self, Write};
use pinboard::{API, Tags};
use settings::Settings;

#[derive(Debug)]
enum Error {
    Config(config::ConfigError),
}

fn run_app() -> Result<(), Error> {
    let settings = match Settings::new() {
        Ok(ok) => ok,
        Err(err) => return Err(Error::Config(err)),
    };

    println!("{:?}", settings);
    let pinboard = API::new(&settings.api.token);
    let things = pinboard.tags_get();
    println!("{:?}", things);
    Ok(())
}

fn main() {
    ::std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            writeln!(io::stderr(), "Error: {:?}", err).unwrap();
            1
        }
    });
}
