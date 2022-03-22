// TODO Docs
use super::util;
use git2;
use std::{fs, io, process};

pub fn update(sub_match: &clap::ArgMatches) {
    // TODO
    if sub_match.is_present("install") {
        util::stdout("info", "Starting Node dependencies installation...");
        // TODO do something with the error code
        let exit_code = node_install();

        // TODO Learn more about this guard please
        let exit_code = match exit_code {
            Ok(code) => code,
            Err(error) => {
                util::stdout("error", &format!("{}", error));
                return;
            }
        };

        if exit_code.success() {
            util::stdout("success", "Node dependencies installed.");
        } else {
            util::stdout("error", "Node dependencies failed to install.");
        }

        // TODO print a "installation finished" message
    }
    // TODO replace this assignments for matches
    let content = sub_match.is_present("content");
    let source = sub_match.is_present("source");
    let assets = sub_match.is_present("assets");
    let data = sub_match.is_present("data");

    // TODO add a case for no matches and do everything
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
