fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir().unwrap();
    find_subdirectories(&current_dir)?;
    Ok(())
}

use std::fs;
use std::io::Write;
use std::path::Path;

fn find_subdirectories(root_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(root_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(subdir_name) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if !subdir_name.starts_with("day-") {
            continue;
        }
        process_package(&path, entry.file_name().to_str().unwrap())?;
    }

    Ok(())
}

use regex::Regex;

fn process_package(path: &Path, year: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cargo_file = path.join("Cargo.toml");

    let cargo_contents = fs::read_to_string(&cargo_file)?;
    let package_name_pattern = Regex::new(r#"name\s*=\s*"(day-\d+)""#)?;

    let replacer = |captures: &regex::Captures| -> String {
        let capture = captures.get(1).unwrap();
        let result = format!("{}-{}", capture.as_str(), year);
        result
    };

    let updated_cargo_contents =
        package_name_pattern.replace_all(cargo_contents.as_str(), replacer);

    let mut new_cargo_file = fs::File::create(&path)?;
    new_cargo_file.write_all(updated_cargo_contents.as_bytes())?;

    Ok(())
}
