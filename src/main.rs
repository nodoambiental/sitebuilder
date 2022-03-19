use clap::{Arg, ArgMatches, Command};
//use slog::Drain;
use std::env;
use std::fs;
use std::io;
use std::path;
use std::process;

fn create_reldir(rel_path: &str) -> Result<(), io::Error> {
    // TODO Check env docs
    // TODO Check what the heck unwrap does and why is not recommended
    let cwd = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    // TODO Check PathBuf docs
    // TODO Check iter(), Iterator and according traits docs
    // TODO Check collect docs
    // TODO Check String::from docs
    let temp_path: path::PathBuf = [cwd, String::from(rel_path)].iter().collect();

    // TODO Check in more detail how borrowing works
    fs::create_dir_all(temp_path).unwrap();

    // HACK add error handling
    Ok(())
}

fn delete_reldir(rel_path: &str) -> Result<(), io::Error> {
    let cwd = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let temp_path: path::PathBuf = [cwd, String::from(rel_path)].iter().collect();

    fs::remove_dir(temp_path).unwrap();

    // HACK add error handling
    Ok(())
}

fn run(matches: ArgMatches) -> Result<(), String> {
    // TODO do stuff

    create_reldir("temp").unwrap();
    println!("Created temp directory");
    delete_reldir("temp").unwrap();
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
                .about("Pulls the data, content and assets from the remote repository.")
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
                .after_help(
                    "By default, this command will pull all the data, content and assets from the remote repositories.",
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
