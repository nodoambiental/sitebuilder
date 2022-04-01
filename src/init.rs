use super::util;
use std::{fs, path};

pub fn init(_sub_match: &clap::ArgMatches) {
    let mut path = path::PathBuf::new();
    path.push(util::cwd_string());

    util::verify_relfile_fatal(
        "",
        "sitebuilder",
        "toml",
        false,
        "The SiteBuilder config file is already present on this folder.",
    );

    path.push("sitebuilder");
    path.set_extension("toml");

    let mut content = String::new();
    content.push_str("# ============================\n");
    content.push_str("#    SiteBuilder config file\n");
    content.push_str("# ============================\n");
    content.push_str("\n# Add the URIs to your sources.\n");
    content.push_str(option_line("assets").as_str());
    content.push_str(option_line("data").as_str());
    content.push_str(option_line("content").as_str());
    content.push_str(option_line("source").as_str());
    content.push_str("\n# output = \"dist\" # Specify output directory for the built site.\n");

    // HACK add error handling
    fs::write(path, content);
}

fn option_line(option: &str) -> String {
    let mut line = String::new();
    line.push_str(
        format!(
            "{} = \"<URI to the git repo containing your {}>\"\n",
            option, option
        )
        .as_str(),
    );
    line
}
