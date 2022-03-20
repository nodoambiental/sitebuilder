use clap::{Arg, ArgMatches, Command};
//use slog::Drain;

use std::process;

mod build;
mod update;
mod util;

fn run(matches: ArgMatches) -> Result<(), String> {
    // TODO do stuff

    util::create_reldir("temp").unwrap();
    println!("Created temp directory");
    util::delete_reldir("temp").unwrap();
    println!("Deleted temp directory");
    // HACK add error handling
    Ok(())
}

fn main() {
    let matches = Command::new("sitebuilder")
        .version("0.2.0")
        .author("Ágata Ordano")
        .about("Builds a reactive, interactive, full blown site from a static template")
        .subcommand(
            Command::new("build")
                .about("Executes the build process.")
                .after_help(
                    "This command will build the bundle, process it, and run it through Zola.",
                )
                .arg(
                    Arg::new("development")
                        .short('D')
                        .long("development")
                        .help("Performs a development build"),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Pulls the data, sourec, content and assets from the remote repositories.")
                .arg(Arg::new("content")
                    .short('c')
                    .long("content")
                    .help("Set this flag to only pull content."))
                .arg(Arg::new("data")
                    .short('d')
                    .long("data")
                    .help("Set this flag to only pull data"))
                .arg(Arg::new("assets")
                    .short('a')
                    .long("assets")
                    .help("Set this flag to only pull assets"))
                .arg(Arg::new("source")
                    .short('s')
                    .long("source")
                    .help("Set this flag to only pull and build the source"))
                .arg(Arg::new("dependencies")
                    .short('i')
                    .long("install")
                    .help("Set this flag to only install dependencies"))
                .after_help(
                    "By default, this command will pull all the data, content, source and assets from the remote repositories, and install the required dependencies.",
                ),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .multiple_occurrences(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();
    if let Err(e) = run(matches) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
