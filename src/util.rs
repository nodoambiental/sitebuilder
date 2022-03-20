use std::env;
use std::fs;
use std::io;
use std::path;

pub fn create_reldir(rel_path: &str) -> Result<(), io::Error> {
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
    let temp_path: path::PathBuf = [cwd, String::from(rel_path)].iter().collect();

    // TODO Check in more detail how borrowing works
    fs::create_dir_all(temp_path).unwrap();

    // HACK add error handling
    Ok(())
}

pub fn delete_reldir(rel_path: &str) -> Result<(), io::Error> {
    let cwd = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let temp_path: path::PathBuf = [cwd, String::from(rel_path)].iter().collect();

    fs::remove_dir(temp_path).unwrap();

    // HACK add error handling
    Ok(())
}
