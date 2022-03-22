// TODO Docs
use super::util;
use git2;
use std::{fs, io, process};

pub fn update(sub_match: &clap::ArgMatches) {
    // TODO massage those warnings
    // TODO add proper error handling
    if sub_match.is_present("install") {
        util::call_with_stdout(
            node_install(),
            "Starting Node dependencies installation...",
            "Node dependencies installed.",
            "Node dependencies failed to install.",
        );
    }

    let mut empty_flag = true;

    if sub_match.is_present("content") {
        empty_flag = false;
        run_update("content");
    }

    if sub_match.is_present("source") {
        empty_flag = false;
        run_update("source");
    }

    if sub_match.is_present("assets") {
        empty_flag = false;
        run_update("assets");
    }

    if sub_match.is_present("data") {
        empty_flag = false;
        run_update("data");
    }

    if empty_flag {
        run_update("content");
        run_update("assets");
        run_update("data");
        run_update("source");
        util::call_with_stdout(
            node_install(),
            "Starting Node dependencies installation...",
            "Node dependencies installed.",
            "Node dependencies failed to install.",
        );
    }
}

fn run_update(target: &str) -> Result<(), io::Error> {
    // TODO Handle error here and return only ()
    // Verify the `source` directory exists
    let source_exists = util::verify_reldir(target).unwrap();
    if source_exists {
        // TODO
        // pull(target);
    } else {
        // TODO
        // clone(target);
    }
    // HACK add error handling
    Ok(())
}

pub fn pull(repo_uri: &str) {
    // TODO
    // Function that verifies that the folders are present, they are Git repos, and updates them
}

pub fn clone(repo_uri: &str) {
    // TODO
    // Function that verifiers that the folders are not present and clones them
}

pub fn node_install() -> Result<process::ExitStatus, io::Error> {
    // TODO add color to the CLI output

    // TODO improve error handling and not just panic senselessly

    // Verify the `source` directory exists
    let source_exists = util::verify_reldir("source").unwrap();
    assert!(
        matches!(source_exists, true),
        "[FS] Source folder does not exist"
    );

    // Verify package.json exists
    let package_exists = util::verify_relfile("source", "package", "json").unwrap();
    assert!(
        matches!(package_exists, true),
        "[FS] There is no package.json file in the source folder"
    );

    // Install modules
    // TODO hide this depending on verbosity level
    let code = process::Command::new("npm")
        .arg("install")
        .current_dir(util::cwd_string() + "/source")
        .spawn()
        .expect("[NPM] ")
        .wait();

    return code;
}
