use super::util;
use git2;

pub fn update(flags: [&str; 5]) {
    // TODO
    // Actual function that reads flags present and calls the proper methods
}

pub fn pull(repo_uri: &str) {
    // TODO
    // Function that verifies that the folders are present, they are Git repos, and updates them
}

pub fn clone(repo_uri: &str) {
    // TODO
    // Function that verifiers that the folders are not present and clones them
}

pub fn node_install() {
    // TODO
    // Function that performs the node_modules dependencies installation
}
