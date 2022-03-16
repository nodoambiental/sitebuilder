use std::env;
use std::fs;
use std::path;

fn main() {
    // TODO check out DirBuilder docs
    let mut dir_maker = fs::DirBuilder::new();
    dir_maker.recursive(true);

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
    let temp_path: path::PathBuf = [cwd, String::from("temp")].iter().collect();

    // TODO Check in more detail how borrowing works
    dir_maker.create(temp_path).unwrap();
}
