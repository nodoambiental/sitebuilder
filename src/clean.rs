// TODO Docs
use super::util;
use colored::*;
use config;
use std::process::ExitStatus;
use std::{fs, io, path, process};

pub fn clean(sub_match: &clap::ArgMatches) {
    // TODO massage those warnings
    // TODO add proper error handling
    // TODO implement custom config path option
    // TODO add an option to only delete intermediary files and not final distribution files

    // HACK add error handling

    let config = util::read_config().unwrap();

    match sub_match.subcommand() {
        Some(("pulled", sub_sub_match)) => {
            util::stdout("info", "Cleaning pulled files...");
            if sub_sub_match.is_present("content") {
                util::stdout("info", "Cleaning content...");
                util::call_with_stdout(
                    clean_sources("content"),
                    "Cleaned content files",
                    "Failed to clean content files",
                );
            }
            if sub_sub_match.is_present("source") {
                util::stdout("info", "Cleaning source...");
                util::call_with_stdout(
                    clean_sources("source"),
                    "Cleaned source files",
                    "Failed to clean source files",
                );
            }
            if sub_sub_match.is_present("assets") {
                util::stdout("info", "Cleaning assets...");
                util::call_with_stdout(
                    clean_sources("assets"),
                    "Cleaned assets",
                    "Failed to clean assets",
                );
            }
            if sub_sub_match.is_present("data") {
                util::stdout("info", "Cleaning data...");
                util::call_with_stdout(
                    clean_sources("data"),
                    "Cleaned data files",
                    "Failed to clean data files",
                );
            }
            util::stdout("info", "...sources cleaning done");
        }
        Some(("generated", sub_sub_match)) => {
            util::stdout("info", "Cleaning generated files...");
            let parsed_outdir = util::get_outdir(&config, sub_sub_match);
            if sub_sub_match.is_present("output") {
                util::call_with_stdout(
                    clean_generated(parsed_outdir.as_str()),
                    "Cleaned generated files",
                    "Failed to clean generated files",
                );
            } else {
                util::call_with_stdout(
                    clean_generated(parsed_outdir.as_str()),
                    "Cleaned generated files",
                    "Failed to clean generated files",
                );
            }
        }
        _ => {}
    }

    if sub_match.is_present("pulled_all") {
        util::stdout("info", "Cleaning all pulled files...");

        util::call_with_stdout(
            clean_sources("content"),
            "Cleaned content files",
            "Failed to clean content files",
        );
        util::stdout("info", "Cleaning source...");
        util::call_with_stdout(
            clean_sources("source"),
            "Cleaned source files",
            "Failed to clean source files",
        );
        util::stdout("info", "Cleaning assets...");
        util::call_with_stdout(
            clean_sources("assets"),
            "Cleaned assets",
            "Failed to clean assets",
        );
        util::stdout("info", "Cleaning data...");
        util::call_with_stdout(
            clean_sources("data"),
            "Cleaned data files",
            "Failed to clean data files",
        );

        util::stdout("info", "...sources cleaning done");
    }
}

fn clean_sources(folder_name: &str) -> Result<process::ExitStatus, io::Error> {
    // Verify the `source` directory exists
    let source_exists = util::verify_reldir(folder_name, true);
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

pub fn clean_generated(outdir: &str) -> Result<process::ExitStatus, io::Error> {
    // Verify the `source` directory exists
    let source_exists = util::verify_reldir(outdir, true);

    if !source_exists {
        util::stdout(
            "warning",
            format!(
                "Generated folder {} not found, nothing to clean",
                outdir.italic()
            )
            .as_str(),
        );
        return Ok(std::os::unix::process::ExitStatusExt::from_raw(0));
    }

    let target: path::PathBuf = [util::cwd_string(), String::from(outdir)].iter().collect();

    //let code = process::Command::new("echo")
    let code = process::Command::new("rm")
        .arg("-rfd")
        .arg(outdir)
        .spawn()
        .expect("[FS] ")
        .wait();
    return code;
}
