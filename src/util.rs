use config;
use std::io::Write;
use std::{env, fs, io, path, process};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn stdout(selector: &str, message: &str) {
    // TODO implement debug level selection
    let mut out = StandardStream::stdout(ColorChoice::Always);
    // TODO implement IO error handling
    match selector {
        "info" => {
            out.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
            writeln!(out, "{}", message);
            out.reset();
        }
        "error" => {
            out.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_intense(true));
            writeln!(out, "{}", message);
            out.reset();
        }
        "warning" => {
            out.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
            writeln!(out, "{}", message);
            out.reset();
        }
        "success" => {
            out.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
            writeln!(out, "{}", message);
            out.reset();
        }
        _ => {
            out.reset();
        }
    }
}

pub fn call_with_stdout(
    exit_code: Result<process::ExitStatus, io::Error>,
    start_message: &str,
    success_message: &str,
    error_message: &str,
) -> bool {
    stdout("info", start_message);

    // TODO Learn more about this guard please
    let exit_code = match exit_code {
        Ok(code) => code,
        Err(error) => {
            stdout("error", &format!("{}", error));
            return false;
        }
    };

    if exit_code.success() {
        stdout("success", success_message);
        return true;
    } else {
        stdout("error", error_message);
        return false;
    }
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

pub fn read_config() -> Result<config::Config, config::ConfigError> {
    // load and return the config
    let config = config::Config::builder()
        .add_source(config::File::new("sitebuilder", config::FileFormat::Toml))
        .build();
    return config;
}

pub fn read_config_custom(config_path: &str) -> Result<config::Config, config::ConfigError> {
    // Check if a custom path is set and build the PathBuf
    let mut path = path::PathBuf::new();

    path.push(config_path);
    path.push("sitebuilder");

    // load and return the config
    let config = config::Config::builder()
        .add_source(config::File::new(
            path.to_str().unwrap(),
            config::FileFormat::Toml,
        ))
        .build();
    return config;
}
