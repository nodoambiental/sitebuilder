use config;
use std::io::Write;
use std::{env, fs, io, path};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn stdout(selector: &str, message: &str) -> Result<(), io::Error> {
    // TODO implement debug level selection
    let mut out = StandardStream::stdout(ColorChoice::Always);
    match selector {
        "info" => {
            out.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
            writeln!(out, "{}", message)?;
            out.reset()?;
        }
        "error" => {
            out.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_intense(true));
            writeln!(out, "{}", message)?;
            out.reset()?;
        }
        "warning" => {
            out.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
            writeln!(out, "{}", message)?;
            out.reset()?;
        }
        "success" => {
            out.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
            writeln!(out, "{}", message)?;
            out.reset()?;
        }
        _ => {
            out.reset()?;
        }
    }
    Ok(())
}

pub fn cwd_string() -> String {
    env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn create_reldir(rel_path: &str) -> Result<(), io::Error> {
    let temp_path: path::PathBuf = [cwd_string(), String::from(rel_path)].iter().collect();

    fs::create_dir_all(temp_path).unwrap();

    // HACK add error handling
    Ok(())
}

pub fn delete_reldir(rel_path: &str) -> Result<(), io::Error> {
    let temp_path: path::PathBuf = [cwd_string(), String::from(rel_path)].iter().collect();
    fs::remove_dir(temp_path).unwrap();

    // HACK add error handling
    Ok(())
}

pub fn verify_reldir(rel_path: &str) -> io::Result<bool> {
    let mut path = env::current_dir()?;
    path.push(rel_path);
    let metadata = fs::metadata(path)?;
    Ok(metadata.is_dir())
}

pub fn verify_relfile(rel_path: &str, name: &str, ext: &str) -> io::Result<bool> {
    let mut fullpath = path::PathBuf::new();
    fullpath.push(cwd_string());
    fullpath.push(rel_path);
    fullpath.push(name);
    fullpath.set_extension(ext);
    let metadata = fs::metadata(fullpath.to_str().unwrap())?;
    Ok(metadata.is_file())
}

pub fn read_config(config_path: &str) -> Result<(), io::Error> {
    // TODO

    // HACK add error handling
    Ok(())
}
