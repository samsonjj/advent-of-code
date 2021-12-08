use regex::Regex;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "aoc-init",
    about = "automatically initializes Rust projects for Advent Of Code"
)]
struct Cli {
    #[structopt(short, long)]
    year: i32,
    #[structopt(short, long)]
    day: i32,
    #[structopt(env = "AOC_SESSION")]
    session: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();
    create_project(&cli).await?;
    Ok(())
}

fn add_to_cargo_workspace() -> Result<(), Box<dyn std::error::Error>> {
    let cargo_toml = std::fs::read_to_string("Cargo.toml")?;
    let mut result = String::new();

    let re = Regex::new(r"/^(.*)members[\s]*=[\s]*\[(.*)\](.*)$/s");

    // let cargo_toml_new = cargo_toml
    //     .lines()
    //     .map(|line| if line.contains("members") {})
    //     .collect();
    Ok(())
}

async fn create_project(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", 1);
    let dir_name = format!("day-{}", cli.day);
    println!("{}", 2);
    Command::new("sh")
        .arg("-c")
        .arg(format!("cargo new --bin {}", &dir_name))
        .output()?;
    println!("{}", 3);
    let input_file_path = PathBuf::from(&dir_name).join("src").join("input.txt");
    println!("about to write to {:?}", input_file_path);
    std::fs::File::create(&input_file_path)?
        .write_all(get_puzzle_input(cli).await?.as_bytes())
        .expect(format!("failed to write to file at {:?}", &input_file_path).as_str());
    println!("{}", 4);
    Ok(())
}

async fn get_puzzle_input(cli: &Cli) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        cli.year, cli.day
    );
    let client = reqwest::Client::new();
    let resp: String = client
        .get(url)
        .header(reqwest::header::COOKIE, format!("session={};", cli.session))
        .send()
        .await?
        .text()
        .await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex() {
        let re = Regex::new(r"^(.*)members[\s]*=[\s]*\[(.*)\](.*)$\n").unwrap();
        let s = String::from(
            "hello
                
            members = [
                1,2,3
            ]

            goodbye
            ",
        );

        let mut iter = re.captures_iter(&s[..]);
        assert_eq!(
            iter.next().unwrap().get(1).unwrap().as_str(),
            "hello

"
        );
        assert_eq!(
            iter.next().unwrap().get(2).unwrap().as_str().trim(),
            "1,2,3"
        );
        assert_eq!(iter.next().unwrap().get(3).unwrap().as_str(), "goodbye");
    }
}
