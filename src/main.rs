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

static DEFAULT_CONFIG_FILE: &'static str = "~/.config/pinboard/config.yaml";

fn notes(p: &pinboard::API, m: &clap::ArgMatches) {
    if m.is_present("list") {
        let notes = p.notes();

        println!("Notes: {:?}\n", notes);
        print!("Notes: ");
        match notes {
            Ok(notes) => {
                for note in notes.notes {
                    println!("{:?}", p.note(&note.id))
                }
            },
            Err(_e) => (),
        }
    }
}

fn post(p: &pinboard::API, m: &clap::ArgMatches) {
    if let Some(m) = m.subcommand_matches("add") {
        // Required param
        let url = m.value_of("url").unwrap();
        println!("Got: {:?}", url);
    }
}

fn tags(p: &pinboard::API, m: &clap::ArgMatches) {
    if m.is_present("list") {
        let tags = p.tags();
        println!("Tags: {:?}\n", tags);
    }
}

fn run_app() -> Result<(), Error> {
    let matches = clap::App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(clap::Arg::with_name("config")
             .short("c")
             .long("config")
             .takes_value(true)
             .env("PINBOARD_CONFIG_FILE")
             .default_value(DEFAULT_CONFIG_FILE)
             .help("Specify a custom config file to load"))
        .arg(clap::Arg::with_name("debug")
             .short("d")
             .long("debug")
             .help("Enable debug output"))
        .subcommand(clap::SubCommand::with_name("notes")
                    .about("Notes related commands")
                    .arg(clap::Arg::with_name("list")
                         .short("l")
                         .long("list")
                         .help("List notes"))
                    )
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
                         .help("List tags"))
                    )
        .get_matches();

    // We supply a default for this value, so it should be safe to unwrap.
    let config_file = matches.value_of("config").unwrap();
    let settings = match Settings::new(&config_file) {
        Ok(ok) => ok,
        Err(err) => return Err(Error::Config(err)),
    };

    let pinboard = API::new(&settings.api.token);

    // This will become unmanagable, we should split this into functions soon.
    match matches.subcommand() {
        ("notes", Some(m)) => notes(&pinboard, &m),
        ("post", Some(m)) => post(&pinboard, &m),
        ("tags", Some(m)) => tags(&pinboard, &m),
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
