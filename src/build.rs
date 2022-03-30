// TODO
// TODO implement the scripting that replaces npm scripts
// TODO translate the tooling.ts files here
// TODO implement the config loading for all the subsystems of the toolchain (webpack, zola, etc)
// TODO replace the non-JS dependent parts of the scripts like moving files and stuff
// TODO implement verbosity levels and pretty-printing
// TODO in some remote future implement synchronization to enable a watch mode
// TODO Docs
use super::clean;
use super::util;
use config;
use pug;
use std::{fs, io, path, process};

pub fn build(sub_match: &clap::ArgMatches) {
    let config = util::read_config();

    let config = match config {
        Ok(config) => config,
        Err(err) => {
            util::stdout("fatal", &format!("Config error: {}", err));
            return;
        }
    };

    let parsed_outdir = util::get_outdir(&config, sub_match);

    // TODO massage those warnings
    // TODO add proper error handling
    // TODO implement custom config path option

    if sub_match.is_present("development") {
        util::stdout("info", "Performing a development build...");
        util::call_with_stdout(
            build_development(parsed_outdir.as_str()),
            "Built successfully",
            "Failed to build site",
        );
    } else {
        util::stdout("info", "Performing a production build...");
        util::call_with_stdout(
            build_production(parsed_outdir.as_str()),
            "Built successfully",
            "Failed to build site",
        );
    }
}

fn build_development(outdir: &str) -> Result<process::ExitStatus, io::Error> {
    // TODO handle status and fatal or continue
    prebuild(&outdir);

    // TODO process pug
    // TODO process SCSS/CSS/PostCSS y Tailwind and stuff
    // TODO process tsc
    // TODO process bundling and stuff
    // TODO cleanup
    // TODO process zola
    // TODO cleanup
    // TODO move to dist

    // ? placeholder to not kill anything
    let code = process::Command::new("echo")
        .arg("-rfd")
        .spawn()
        .expect("[FS] ")
        .wait();

    return code;
}

fn build_production(outdir: &str) -> Result<process::ExitStatus, io::Error> {
    // TODO handle status and fatal or continue
    prebuild(&outdir);

    // TODO process pug
    // TODO process SCSS/CSS/PostCSS y Tailwind and stuff
    // TODO process tsc
    // TODO process bundling and stuff
    // TODO cleanup
    // TODO process zola
    // TODO cleanup
    // TODO move to dist

    // ? placeholder to not kill anything
    let code = process::Command::new("echo")
        .arg("-rfd")
        .spawn()
        .expect("[FS] ")
        .wait();

    return code;
}

fn prebuild(outdir: &str) -> bool {
    let mut all_ok = true;
    all_ok = util::verify_reldir_fatal("source", "Try pulling source folder first");
    all_ok = util::verify_reldir_fatal("assets", "Try pulling assets first");
    all_ok = util::verify_reldir_fatal("data", "Try pulling data first");
    all_ok = util::verify_reldir_fatal("content", "Try pulling content first");
    all_ok = util::verify_relfile_fatal(
        "source",
        "package",
        "json",
        "Cannot install dependencies without a package.json file",
    );

    if !all_ok {
        return false;
    }

    // TODO
    copy_static(outdir);

    // HACK add result and error
    return true;
}

fn copy_static(outdir: &str) -> bool {
    // HACK add error handling
    // TODO add optionality to cleanup
    clean::clean_generated(outdir);

    util::create_reldir(outdir);

    // TODO Copy and locate movable assets
    // TODO Copy and locate movable content
    // TODO Copy and locate movable data
    // TODO Copy and locate remaining stuff
    // TODO Cleanup and handle errors

    // HACK add result and error
    return true;
}
