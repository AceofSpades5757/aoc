use clap::Parser;
use clap::Subcommand;
use colored::*;
use serde::{Deserialize, Serialize};

/// Advent of Code command line tool to facilitate solving puzzles.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Automatically retrieve input file, based on the current working directory's day: day-XX/input.txt
    Input,
    /// Submit answer, based on the current working directory's day.
    Submit {
        #[clap(short, long)]
        input: String,
    },
    /// Create a new day, based on previous days, up to 25.
    Day,
    /// Copy part 1 to part 2, based on the current working directory's day.
    Part,
}

struct Environment {
    day: Option<u8>,
    year: u16,
}

#[derive(Debug)]
enum EnvironmentError {
    InvalidYear,
}

#[derive(Debug)]
enum Error {
    EnvironmentError(EnvironmentError),
}

/// Constructors
impl Environment {
    fn new(day_format: &str, year_format: &str) -> Result<Self, Error> {
        let day: Option<u8>;
        let year: u16;

        let current_dir: String = std::env::current_dir()
            .unwrap()
            .to_owned()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let parent_dir: String = std::env::current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .to_owned()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        if parent_dir.contains(year_format) {
            year = parent_dir.replace(year_format, "").parse().unwrap();
            day = Some(current_dir.replace(day_format, "").parse().unwrap());
        } else {
            if !current_dir.contains(year_format) {
                return Err(Error::EnvironmentError(EnvironmentError::InvalidYear));
            } else {
                year = current_dir.replace(year_format, "").parse().unwrap();
                day = None
            }
        }

        Ok(Environment { day, year })
    }
}

/// Validators
impl Environment {
    fn check_day(day_format: &str) -> Result<(), String> {
        let current_dir: String = std::env::current_dir()
            .unwrap()
            .to_owned()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        if !current_dir.contains(day_format) {
            Err(format!(
                "Current directory not valid, <{}>. Should look like <{}>",
                current_dir, day_format
            ))
        } else {
            Ok(())
        }
    }
    fn check_year(year_format: &str) -> Result<(), String> {
        let current_dir: String = std::env::current_dir()
            .unwrap()
            .to_owned()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let parent_dir: String = std::env::current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .to_owned()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        if !parent_dir.contains(year_format) && !current_dir.contains(year_format) {
            Err(format!(
                "Parent directory not valid: {}. Should look like <{}>",
                parent_dir, year_format
            ))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Formats {
    day: Option<String>,
    year: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    formats: Formats,
}

fn main() {
    // Config
    let config: Config = toml::from_str(include_str!("../config.toml")).unwrap();

    let day_format: String;
    let year_format: String;
    if config.formats.day.is_none() {
        day_format = "day-".to_owned();
    } else {
        day_format = config.formats.day.unwrap();
    }
    if config.formats.year.is_none() {
        year_format = "advent-of-code-".to_owned();
    } else {
        year_format = config.formats.year.unwrap();
    }

    // Environment
    let environment: Environment;
    match Environment::new(&day_format, &year_format) {
        Ok(env) => environment = env,
        Err(e) => {
            eprintln!("{}", format!("Invalid environment: {:?}", e).red());
            std::process::exit(1);
        }
    }

    // Commands
    let args = Args::parse();
    match args.action {
        Action::Input => {
            // Check CWD
            helpers::check_day_and_year_dirs(&day_format, &year_format);
            let input = get_input(environment.year, environment.day.unwrap());
            let result = std::fs::write("input.txt", input);
            if result.is_ok() {
                println!("{}", "Success".green());
            } else {
                println!(
                    "{}",
                    format!("Failed to write input file: {:?}", result).red()
                );
            }
        }
        Action::Submit { input } => {
            // Check CWD
            helpers::check_day_and_year_dirs(&day_format, &year_format);
            println!("Submit: {}", input);
        }
        Action::Day => {
            // Check CWD
            helpers::check_year_dir(&year_format);
            // New Day Directory Name
            let mut highest_day: u8 = 0;
            for entry in std::fs::read_dir(".").unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let path_str = path.file_name().unwrap().to_str().unwrap();
                if path_str.contains(&day_format) {
                    let day: u8 = path_str.replace(&day_format, "").parse().unwrap_or(0);
                    if day > highest_day {
                        highest_day = day;
                    }
                }
            }
            let new_day: u8 = highest_day + 1;
            let new_day_str: String = format!("{}{:02}", day_format, new_day);
            // Create new day directory
            let result = std::fs::create_dir(&new_day_str);
            if result.is_ok() {
                println!("New Project Directory: {}", "Success".green());
            } else {
                println!(
                    "{}",
                    format!("Failed to create new day directory: {:?}", result).red()
                );
            }
            // copy template from ./templates/Cargo.toml
            let template_cargo_toml = include_str!("../templates/Cargo.toml");
            let template_cargo_toml = template_cargo_toml.replace(r#"name = """#, &format!(r#"name = "{}""#, &new_day_str));
            let result = std::fs::write(format!("{}/Cargo.toml", new_day_str), template_cargo_toml);
            if result.is_ok() {
                println!("New Cargo.toml: {}", "Success".green());
            } else {
                println!(
                    "{}",
                    format!("Failed to create new Cargo.toml: {:?}", result).red()
                );
            }
            // mkdir for src
            let result = std::fs::create_dir(format!("{}/src", new_day_str));
            if result.is_ok() {
                println!("New src Directory: {}", "Success".green());
            } else {
                println!(
                    "{}",
                    format!("Failed to create new src directory: {:?}", result).red()
                );
            }
            // mkdir for src/bin
            let result = std::fs::create_dir(format!("{}/src/bin", new_day_str));
            if result.is_ok() {
                println!("New src/bin Directory: {}", "Success".green());
            } else {
                println!(
                    "{}",
                    format!("Failed to create new src/bin directory: {:?}", result).red()
                );
            }
            // copy template part 1 from ./templates/part1.rs to src/bin/part_1.rs
            let template_part_1 = include_str!("../templates/part.rs");
            let result = std::fs::write(format!("{}/src/bin/part_1.rs", new_day_str), template_part_1);
            if result.is_ok() {
                println!("New src/bin/part_1.rs: {}", "Success".green());
            } else {
                println!(
                    "{}",
                    format!("Failed to create new src/bin/part_1.rs: {:?}", result).red()
                );
            }
        }
        Action::Part => {
            // Check CWD
            helpers::check_day_and_year_dirs(&day_format, &year_format);

            // Check: is there already a src/bin/part_2.rs?
            let part_2_path = "./src/bin/part_2.rs";
            if std::path::Path::new(&part_2_path).exists() {
                println!(
                    "{}",
                    format!("{} already exists.", part_2_path).red()
                );
                std::process::exit(1);
            }

            // Run: `cp src/bin/part_1.rs src/bin/part_2.rs`
            let child = std::process::Command::new("cp")
                .arg(format!("src/bin/part_1.rs"))
                .arg(format!("src/bin/part_2.rs"))
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .expect("Run cp command");
            let output = child.wait_with_output().expect("cp command finished");

            if output.status.success() {
                println!("{}", "Success".green());
            } else {
                println!(
                    "{}",
                    format!(
                        "Failed to copy part 1 to part 2: {:?}",
                        String::from_utf8(output.stderr.as_slice().to_vec())
                    )
                    .red()
                );
            }
        }
    }
}

pub mod helpers {
    use crate::Environment;
    use colored::*;

    pub fn check_day_and_year_dirs(day_format: &str, year_format: &str) {
        // Verify user is in the correct directory.
        // 1. advent-of-code-{year}
        // 2. day-XX
        if let Err(err) = Environment::check_day(day_format) {
            eprintln!("{}", format!("Error: {}", err).red());
            std::process::exit(1);
        }
        if let Err(err) = Environment::check_year(year_format) {
            eprintln!("{}", format!("Error: {}", err).red());
            std::process::exit(1);
        }
    }

    pub fn check_year_dir(year_format: &str) {
        // Verify user is in the correct directory.
        // 1. advent-of-code-{year}
        if let Err(err) = Environment::check_year(year_format) {
            eprintln!("{}", format!("Error: {}", err).red());
            std::process::exit(1);
        }
    }
}

fn get_input(year: u16, day: u8) -> String {
    use dotenv::dotenv;
    use std::env;

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let cwd = std::env::current_dir().unwrap();
    env::set_current_dir("../..").unwrap();
    dotenv().ok();
    env::set_current_dir(cwd).unwrap();

    let session_cookie: String;
    match env::var("session") {
        Ok(val) => session_cookie = val,
        Err(e) => {
            eprintln!("{}", format!("session key not set in .env file: {}", e).red());
            std::process::exit(1);
        }
    }
    let client = reqwest::blocking::Client::new();
    let mut response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_cookie))
        .header("User-Agent", "AceofSpades5757")
        .send()
        .unwrap();

    // if code is 404, try up to 5 times
    let max_tries = 5;
    let mut tries = 0;
    while response.status() == 404 && tries < max_tries {
        eprintln!("{}", "Puzzle has not yet opened, retrying...".yellow());
        response = client
            .get(&url)
            .header("Cookie", format!("session={}", session_cookie))
            .header("User-Agent", "AceofSpades5757")
            .send()
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1_000));
        tries += 1;
    }
    if response.status() == 404 {
        eprintln!(
            "{}",
            "Puzzle has not yet opened, please try again later.".red()
        );
        std::process::exit(1);
    }

    let text = response.text().unwrap();
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_day() {
        // Set Up
        let tmp_dir = std::env::temp_dir();
        let day_dir = tmp_dir.join("day-01");
        std::fs::create_dir_all(&day_dir).unwrap();
        std::env::set_current_dir(&day_dir).unwrap();

        let day_format = "day-";
        assert_eq!(Environment::check_day(day_format), Ok(()));
    }
    #[test]
    fn test_check_year() {
        // Set Up
        let tmp_dir = std::env::temp_dir();
        let year_dir = tmp_dir.join("advent-of-code-2020");
        let day_dir = year_dir.join("day-01");
        std::fs::create_dir_all(&year_dir).unwrap();
        std::fs::create_dir_all(&day_dir).unwrap();
        std::env::set_current_dir(&day_dir).unwrap();

        let year_format = "advent-of-code-";
        assert_eq!(Environment::check_year(year_format), Ok(()));
    }
}
