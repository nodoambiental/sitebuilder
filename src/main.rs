use clap;
//use slog::Drain;

use std::process;

mod build;
mod update;
mod util;

fn run(cli: clap::ArgMatches) -> Result<(), String> {
    // TODO do stuff

    match cli.subcommand() {
        Some(("update", sub_m)) => {
            // TODO
            // Update the project
            update::update(sub_m)
        }
        Some(("build", sub_m)) => {
            // TODO
            // Build the project
            //build::build(sub_m)
        }
        _ => panic!("[ERROR] No subcommand was specified"),
    }
    // HACK add error handling
    Ok(())
}

fn main() {
    let matches = clap::Command::new("sitebuilder")
        .version("0.2.0")
        .author("Ágata Ordano")
        .arg_required_else_help(true)
        .about("Builds a reactive, interactive, full blown site from a static template")
        .subcommand(
            clap::Command::new("build")
                .about("Executes the build process.")
                .after_help(
                    "This command will build the bundle, process it, and run it through Zola.",
                )
                .arg(
                    clap::Arg::new("development")
                        .short('D')
                        .long("development")
                        .help("Performs a development build"),
                ),
        )
        .subcommand(
            clap::Command::new("update")
                .about("Pulls the data, sourec, content and assets from the remote repositories.")
                .arg(clap::Arg::new("content")
                    .short('c')
                    .long("content")
                    .takes_value(false)
                    .help("Set this flag to only pull content."))
                .arg(clap::Arg::new("data")
                    .short('d')
                    .long("data")
                    .takes_value(false)
                    .help("Set this flag to only pull data"))
                .arg(clap::Arg::new("assets")
                    .short('a')
                    .long("assets")
                    .takes_value(false)
                    .help("Set this flag to only pull assets"))
                .arg(clap::Arg::new("source")
                    .short('s')
                    .long("source")
                    .takes_value(false)
                    .help("Set this flag to only pull and build the source"))
                .arg(clap::Arg::new("install")
                    .short('i')
                    .long("install")
                    .takes_value(false)
                    .help("Set this flag to only install dependencies"))
                .after_help(
                    "By default, this command will pull all the data, content, source and assets from the remote repositories, and install the required dependencies.",
                ),
        )
        .arg(
            clap::Arg::new("verbose")
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
