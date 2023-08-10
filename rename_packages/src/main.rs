use env_logger;
use log::{error, info};

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    info!("Starting package renaming process");

    let current_dir = std::env::current_dir().unwrap();
    if let Err(err) = find_subdirectories(&current_dir, None) {
        error!("Error: {:#?}", err);
        error!("Error: {:#?}", err.backtrace());
    }

    Ok(())
}

use std::fs;
use std::io::Write;
use std::path::Path;

fn find_subdirectories<'a>(
    root_dir: &'a Path,
    mut year: Option<&'a str>,
) -> Result<(), anyhow::Error> {
    let year_regex_pattern = Regex::new(r#"\d+"#)?;
    let root_dir_name = root_dir.file_name().unwrap().to_str().unwrap();
    if year_regex_pattern.is_match(root_dir_name) {
        year = Some(root_dir_name);
    }

    for entry in fs::read_dir(root_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        // info!("Checking {}", path.display());
        if !path.is_dir() {
            continue;
        }
        let Some(subdir_name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if subdir_name.starts_with("day-") && year.is_some() {
            info!("Processing {}, {}", subdir_name, year.unwrap());
            process_package(&path, year.unwrap())?;
        } else {
            find_subdirectories(&path, year)?;
        }
        process_package(&path, entry.file_name().to_str().unwrap())?;
    }

    Ok(())
}

use regex::Regex;

fn process_package(path: &Path, year: &str) -> Result<(), anyhow::Error> {
    let cargo_file = path.join("Cargo.toml");

    let Ok(cargo_contents) = fs::read_to_string(&cargo_file) else {
        return Ok(());
    };
    let package_name_pattern = Regex::new(r#"name\s*=\s*"(day-\d+)""#)?;

    let replacer = |captures: &regex::Captures| -> String {
        let capture = captures.get(1).unwrap();

        let result = format!(r#"name = "{}-{}""#, capture.as_str(), year);
      
        result
    };

    let updated_cargo_contents =
        package_name_pattern.replace_all(cargo_contents.as_str(), replacer);

    let mut new_cargo_file = fs::File::create(&cargo_file)?;
    new_cargo_file.write_all(updated_cargo_contents.as_bytes())?;

    Ok(())
}
