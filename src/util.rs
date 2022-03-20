use config;
use std::env;
use std::fs;
use std::io;
use std::path;

pub fn create_reldir(rel_path: &str) -> Result<(), io::Error> {
    let cwd = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let temp_path: path::PathBuf = [cwd, String::from(rel_path)].iter().collect();

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

pub fn read_config(config_path: &str) -> Result<(), io::Error> {
    // TODO

    // HACK add error handling
    Ok(())
}
