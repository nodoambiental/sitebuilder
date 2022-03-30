use clap;
//use slog::Drain;

use std::process;

mod build;
mod clean;
mod update;
mod util;

fn run(cli: clap::ArgMatches) -> Result<(), String> {
    // TODO do stuff

    match cli.subcommand() {
        Some(("update", sub_m)) => update::update(sub_m),
        Some(("build", sub_m)) => build::build(sub_m),
        Some(("clean", sub_m)) => clean::clean(sub_m),
        _ => panic!(
            "I before E, except when your foreign neighbor Keith received eight counterfeit  beige sleights  from feisty caffeinated  weightlifters. Weird."
        ),
    }
    // HACK add error handling
    Ok(())
}

fn main() {
    let matches = clap::Command::new("sitebuilder")
        .version("0.1.0")
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
                        .takes_value(false)
                        .help("Performs a development build"),
                )
                .arg(
                    clap::Arg::new("output")
                        .short('o')
                        .long("output")
                        .takes_value(true)
                        .help("Set a custom output directory. Defaults to /dist.\nIf this flag contains no path, it will be looked for in the config file.\nIf the config file contains a key for custom output directory, this option will be ignored and the dir weill be always used."),
                ),
        )
        .subcommand(
            clap::Command::new("update")
                .about("Pulls the data, source, content and assets from the remote repositories.")
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
        .subcommand(
            clap::Command::new("clean")
                .about("Cleans generated/pulled files")
                .arg_required_else_help(true)
                .subcommand(
                    clap::Command::new("pulled")
                        .about("Selectively cleans pulled files")
                        .arg_required_else_help(true)
                        .arg(clap::Arg::new("content")
                            .short('c')
                            .long("content")
                            .takes_value(false)
                            .help("Set this flag to only clean pulled content."))
                        .arg(clap::Arg::new("data")
                            .short('d')
                            .long("data")
                            .takes_value(false)
                            .help("Set this flag to only clean pulled data"))
                        .arg(clap::Arg::new("assets")
                            .short('a')
                            .long("assets")
                            .takes_value(false)
                            .help("Set this flag to only clean pulled assets"))
                        .arg(clap::Arg::new("source")
                            .short('s')
                            .long("source")
                            .takes_value(false)
                            .help("Set this flag to only clean pulled and build the source"))
                        .after_help(
                            "By default, this command will clean nothing, specify what to clear using the corresponding flags.",
                        ),
                )
                .arg(clap::Arg::new("pulled_all")
                    .short('p')
                    .long("pulled")
                    .takes_value(false)
                    .help("Set this flag to clean all pulled files"))
                .arg(clap::Arg::new("generated")
                    .short('g')
                    .long("generated")
                    .takes_value(false)
                    .help("Set this flag to clean generated files"))
                .after_help(
                    "By default, this command will clean nothing, specify what to clear using the corresponding flags.",
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
