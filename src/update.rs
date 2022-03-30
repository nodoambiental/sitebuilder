// TODO Docs
use super::util;
use config;
use std::{fs, io, path, process};

pub fn update(sub_match: &clap::ArgMatches) {
    // TODO massage those warnings
    // TODO add proper error handling
    // TODO implement custom config path option

    // HACK add error handling
    let config = util::read_config().unwrap();

    let mut empty_flag = true;

    if sub_match.is_present("install") {
        empty_flag = false;
        // TODO we might need to offload this to the node_install() function so
        // we don't have duplicate error checking code

        util::stdout("info", "Starting Node dependencies installation...");
        util::call_with_stdout(
            node_install(),
            "Node dependencies installed.",
            "Node dependencies failed to install.",
        );
    }

    if sub_match.is_present("content") {
        empty_flag = false;
        run_update(&config, "content");
    }

    if sub_match.is_present("source") {
        empty_flag = false;
        run_update(&config, "source");
    }

    if sub_match.is_present("assets") {
        empty_flag = false;
        run_update(&config, "assets");
    }

    if sub_match.is_present("data") {
        empty_flag = false;
        run_update(&config, "data");
    }

    if empty_flag {
        run_update(&config, "content");
        run_update(&config, "assets");
        run_update(&config, "data");
        run_update(&config, "source");
        util::stdout("info", "Starting Node dependencies installation...");
        util::call_with_stdout(
            node_install(),
            "Node dependencies installed.",
            "Node dependencies failed to install.",
        );
    }
}

fn run_update(config: &config::Config, target: &str) -> Result<(), io::Error> {
    // TODO Handle error here and return only ()
    // Verify the `source` directory exists
    let source_exists = util::verify_reldir(target);
    if source_exists {
        util::stdout("info", format!("Pulling {}...", target).as_str());
        util::call_with_stdout(
            pull(target),
            format!("{} pulled.", target).as_str(),
            format!("{} pull failed. :(", target).as_str(),
        );
    } else {
        util::stdout("info", format!("Cloning {}...", target).as_str());
        util::call_with_stdout(
            clone(config.get_string(target).unwrap().as_str(), target),
            format!("{} cloned.", target).as_str(),
            format!("{} cloning failed. :(", target).as_str(),
        );
    }
    // HACK add error handling
    Ok(())
}

pub fn pull(target: &str) -> Result<process::ExitStatus, io::Error> {
    // Verify git config file exists in repo
    let is_git_repo = util::verify_relfile((String::from(target) + "/.git").as_str(), "config", "");
    assert!(
        matches!(is_git_repo, true),
        "[FS] The target folder is not a git repository"
    );

    // Pull repo
    // TODO hide this depending on verbosity level
    let code_first = process::Command::new("git")
        .arg("fetch")
        .arg("--all")
        .current_dir(util::cwd_string() + "/" + target)
        .spawn()
        .expect("[Git] ")
        .wait();

    if code_first.as_ref().unwrap().success() {
        // TODO hide this depending on verbosity level
        // TODO implement pulling from origin/dev on presence of some devbuild flag
        let code_second = process::Command::new("git")
            .arg("reset")
            .arg("--hard")
            .arg("origin/master") // TODO implement custom branches
            .current_dir(util::cwd_string() + "/" + target)
            .spawn()
            .expect("[Git] ")
            .wait();
        return code_second;
    }
    return code_first;
}

pub fn clone(repo_uri: &str, target: &str) -> Result<process::ExitStatus, io::Error> {
    // Add target dir
    util::create_reldir(target);

    // Clone repo
    // TODO hide this depending on verbosity level
    let code = process::Command::new("git")
        .arg("clone")
        .arg(repo_uri)
        .arg(".")
        .current_dir(util::cwd_string() + "/" + target)
        .spawn()
        .expect("[Git] ")
        .wait();
    return code;
}

pub fn node_install() -> Result<process::ExitStatus, io::Error> {
    // TODO improve error handling and not just panic senselessly

    // Verify the `source` directory exists
    let source_exists = util::verify_reldir("source");
    assert!(
        matches!(source_exists, true),
        "[FS] Source folder does not exist"
    );

    // Verify package.json exists
    let package_exists = util::verify_relfile("source", "package", "json");
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
