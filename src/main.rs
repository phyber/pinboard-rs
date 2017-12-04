extern crate config;
extern crate pinboard;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod settings;

use pinboard::API;
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
    let notes = pinboard.notes();
    let tags = pinboard.tags();

    println!("Notes: {:?}\n", notes);
    println!("Tags: {:?}\n", tags);

    print!("Notes: ");
    match notes {
        Ok(notes) => {
            for note in notes.notes {
                println!("{:?}", pinboard.note(&note.id))
            }
        },
        Err(_e) => (),
    }
    Ok(())
}

fn main() {
    ::std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            1
        }
    });
}
