// TODO
// TODO implement the scripting that replaces npm scripts
// TODO translate the tooling.ts files here
// TODO implement the config loading for all the subsystems of the toolchain (webpack, zola, etc)
// TODO replace the non-JS dependent parts of the scripts like moving files and stuff
// TODO implement verbosity levels and pretty-printing
// TODO in some remote future implement synchronization to enable a watch mode
// TODO Docs

// TODO Implement the builder using SWC instead of Webpack/TSC

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

    util::stdout("info", "Starting webpack...");
    // TODO hide this depending on verbosity level
    let code_first = process::Command::new("webpack")
        .arg("--mode=production")
        .arg("--node-env=production")
        .current_dir(util::cwd_string() + "/source")
        .spawn()
        .expect("[TSC] ")
        .wait();

    if code_first.as_ref().unwrap().success() {
        util::stdout("info", "Webpack bundling done");
        util::stdout("info", "Starting Zola...");

        // TODO hide this depending on verbosity level
        let code_second = process::Command::new("zola")
            .arg("build")
            .arg("--output-dir")
            .arg(outdir)
            .current_dir(util::cwd_string() + "zola_build")
            .spawn()
            .expect("[Zola] ")
            .wait();

        if !code_second.as_ref().unwrap().success() {
            fail("Zola failed to build", outdir);
        }
        return code_second;
    } else {
        fail("Webpack failed to build", outdir);
        return code_first;
    }
}

fn prebuild(outdir: &str) -> bool {
    let source_ok = util::verify_reldir_fatal("source", true, "Try pulling source folder first");
    let assets_ok = util::verify_reldir_fatal("assets", true, "Try pulling assets first");
    let data_ok = util::verify_reldir_fatal("data", true, "Try pulling data first");
    let content_ok = util::verify_reldir_fatal("content", true, "Try pulling content first");
    let package_ok = util::verify_relfile_fatal(
        "source",
        "package",
        "json",
        true,
        "Cannot install dependencies without a package.json file",
    );

    if !source_ok || !assets_ok || !data_ok || !content_ok || !package_ok {
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
    // HACK add error handling
    util::create_reldir(outdir);

    // TODO add custom path support for temp files
    let build_dir = "zola_build";

    // HACK add error handling
    // TODO Better handle cleaning folders and stuff
    clean::clean_generated(build_dir);
    // HACK add error handling
    util::create_reldir(build_dir);
    util::create_reldir(format!("{}/templates", build_dir).as_str());

    // HACK add error handling
    util::symbolic_link_reldir("assets", "static", build_dir);
    // HACK add error handling
    util::symbolic_link_reldir("content", "content", build_dir);
    // HACK add error handling
    util::symbolic_link_reldir("data", "data", build_dir);
    // HACK add error handling
    util::symbolic_link_relfile("data", "config", "toml", build_dir, "");
    // TODO Copy and locate remaining stuff
    // TODO Cleanup and handle errors

    // HACK add result and error
    return true;
}

fn fail(message: &str, outdir: &str) {
    util::stdout("warning", "build failed, cleaned up generated files");

    // TODO add error handling
    clean::clean_generated(outdir);
    // TODO add error handling
    // HACK remove hardcoded string
    clean::clean_generated("zola_build");
    util::stdout("fatal", message);
}
