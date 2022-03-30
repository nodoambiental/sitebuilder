// TODO Docs
use super::util;
use config;
use std::{fs, io, path, process};

pub fn clean(sub_match: &clap::ArgMatches) {
    // TODO massage those warnings
    // TODO add proper error handling
    // TODO implement custom config path option

    // HACK add error handling
    // TODO Handle usage
    let _config = util::read_config().unwrap();

    match sub_match.subcommand() {
        Some(("pulled", sub_sub_match)) => {
            util::stdout("info", "Cleaning pulled files...");
            if sub_sub_match.is_present("content") {
                util::stdout("info", "Cleaning content...");
                util::call_with_stdout(
                    clean_sources("content"),
                    "Cleaned source content files.",
                    "Failed to clean source content files.",
                );
            }
            if sub_sub_match.is_present("source") {
                util::stdout("info", "Cleaning source...");
                util::call_with_stdout(
                    clean_sources("source"),
                    "Cleaned source files.",
                    "Failed to clean source files.",
                );
            }
            if sub_sub_match.is_present("assets") {
                util::stdout("info", "Cleaning assets...");
                util::call_with_stdout(
                    clean_sources("assets"),
                    "Cleaned source asset files.",
                    "Failed to clean source asset files.",
                );
            }
            if sub_sub_match.is_present("data") {
                util::stdout("info", "Cleaning data...");
                util::call_with_stdout(
                    clean_sources("data"),
                    "Cleaned source data files.",
                    "Failed to clean source data files.",
                );
            }
            util::stdout("info", "...sources cleaning done.");
        }
        _ => {}
    }

    if sub_match.is_present("pulled_all") {
        util::stdout("info", "Cleaning all pulled files...");

        util::call_with_stdout(
            clean_sources("content"),
            "Cleaned source content files.",
            "Failed to clean source content files.",
        );
        util::stdout("info", "Cleaning source...");
        util::call_with_stdout(
            clean_sources("source"),
            "Cleaned source files.",
            "Failed to clean source files.",
        );
        util::stdout("info", "Cleaning assets...");
        util::call_with_stdout(
            clean_sources("assets"),
            "Cleaned source asset files.",
            "Failed to clean source asset files.",
        );
        util::stdout("info", "Cleaning data...");
        util::call_with_stdout(
            clean_sources("data"),
            "Cleaned source data files.",
            "Failed to clean source data files.",
        );

        util::stdout("info", "...sources cleaning done.");
    }

    if sub_match.is_present("generated") {
        util::stdout("info", "Cleaning generated files...");
        util::call_with_stdout(
            clean_generated(),
            "Cleaned generated files.",
            "Failed to clean generated files.",
        );
    }
}

fn clean_sources(folder_name: &str) -> Result<process::ExitStatus, io::Error> {
    // Verify the `source` directory exists
    let source_exists = util::verify_reldir(folder_name);
    assert!(matches!(source_exists, true), "{}", folder_name);

    let target: path::PathBuf = [util::cwd_string(), String::from(folder_name)]
        .iter()
        .collect();

    // Install modules
    // TODO hide this depending on verbosity level
    //let code = process::Command::new("echo")
    let code = process::Command::new("rm")
        .arg("-rfd")
        .arg(target.to_str().unwrap())
        .spawn()
        .expect("[FS] ")
        .wait();

    return code;
}

fn clean_generated() -> Result<process::ExitStatus, io::Error> {
    // Verify the `source` directory exists
    let source_exists = util::verify_reldir("build");
    assert!(
        matches!(source_exists, true),
        "[FS] build folder does not exist"
    );

    let target: path::PathBuf = [util::cwd_string(), String::from("build")].iter().collect();

    // Install modules
    // TODO hide this depending on verbosity level

    //let code = process::Command::new("echo")
    let code = process::Command::new("rm")
        .arg("-rfd")
        .arg(target.to_str().unwrap())
        .spawn()
        .expect("[FS] ")
        .wait();

    return code;
}
