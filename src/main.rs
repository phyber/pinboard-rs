extern crate config;
extern crate pinboard;
extern crate serde;

#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;

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

    let matches = clap::App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(clap::Arg::with_name("debug")
             .short("d")
             .long("debug")
             .help("Enable debug output"))
        .subcommand(clap::SubCommand::with_name("post")
                    .about("Post related commands")
                    .subcommand(clap::SubCommand::with_name("add")
                                .about("Add a URL to Pinboard")
                                .arg(clap::Arg::with_name("url")
                                     .short("u")
                                     .long("url")
                                     .required(true)
                                     .takes_value(true)
                                     .help("URL to add"))
                                .arg(clap::Arg::with_name("description")
                                     .short("d")
                                     .long("description")
                                     .required(true)
                                     .takes_value(true)
                                     .help("Description of URL"))
                                )
                    )
        .subcommand(clap::SubCommand::with_name("tags")
                    .about("Tags related commands")
                    .arg(clap::Arg::with_name("list")
                         .short("l")
                         .long("list")
                         .help("List tags")))
        .get_matches();

    println!("{:?}", settings);
    let pinboard = API::new(&settings.api.token);

    // This will become unmanagable, we should split this into functions soon.
    match matches.subcommand() {
        ("notes", Some(m)) => {
            if m.is_present("list") {
                let notes = pinboard.notes();

                println!("Notes: {:?}\n", notes);
                print!("Notes: ");
                match notes {
                    Ok(notes) => {
                        for note in notes.notes {
                            println!("{:?}", pinboard.note(&note.id))
                        }
                    },
                    Err(_e) => (),
                }
            }
        },
        ("post", Some(m)) => {
            if let Some(m) = m.subcommand_matches("add") {
                // Required param
                let url = m.value_of("url").unwrap();
                println!("Got: {:?}", url);
            }
        },
        ("tags", Some(m)) => {
            if m.is_present("list") {
                let tags = pinboard.tags();
                println!("Tags: {:?}\n", tags);
            }
        },
        _ => println!("Unrecognised subcommand"),
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
