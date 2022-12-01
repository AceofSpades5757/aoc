use clap::Parser;
use clap::Subcommand;

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
    Submit{
        #[clap(short, long)]
        input: String,
    },
    /// Create a new day, based on previous days, up to 25.
    Day,
    /// Copy part 1 to part 2, based on the current working directory's day.
    Part,
}

struct Environment {
    day: u8,
    year: u16,
}

/// Constructors
impl Environment {
    fn new() -> Self {
        // directory
        let day_dir: String = std::env::current_dir().unwrap().to_owned().file_name().unwrap().to_str().unwrap().to_owned();
        let year_dir: String = std::env::current_dir().unwrap().parent().unwrap().to_owned().file_name().unwrap().to_str().unwrap().to_owned();

        println!("path: {:?}", year_dir);
        println!("path: {:?}", day_dir);

        let day = 1;
        let year = 2;

        Environment { day, year }
    }
}

/// Validators
impl Environment {
    fn check_day(day_format: &str) -> Result<(), String> {
        let day_dir: String = std::env::current_dir().unwrap().to_owned().file_name().unwrap().to_str().unwrap().to_owned();

        if !day_dir.contains(day_format) {
            Err(format!("Day directory not valid: {}", day_dir))
        } else {
            Ok(())
        }
    }
    fn check_year(year_format: &str) -> Result<(), String> {
        let year_dir: String = std::env::current_dir().unwrap().parent().unwrap().to_owned().file_name().unwrap().to_str().unwrap().to_owned();

        if !year_dir.contains(year_format) {
            Err(format!("Year directory not valid: {}", year_dir))
        } else {
            Ok(())
        }
    }
}

fn main() {
    // Config
    let year_format = "advent-of-code-";
    let day_format = "day-";

    let environ = Environment::new();

    let args = Args::parse();
    match args.action {
        Action::Input => {
            helpers::check_day_and_year_dirs(day_format, year_format);
            println!("Input");
        },
        Action::Submit{input} => {
            helpers::check_day_and_year_dirs(day_format, year_format);
            println!("Submit: {}", input);
        },
        Action::Day => {
            helpers::check_year_dir(year_format);
            println!("Day");
        },
        Action::Part => {
            helpers::check_day_and_year_dirs(day_format, year_format);
            println!("Part")
    },
    }
}

pub mod helpers {
    use crate::Environment;

    pub fn check_day_and_year_dirs(day_format: &str, year_format: &str) {
        // Verify user is in the correct directory.
        // 1. advent-of-code-{year}
        // 2. day-XX
        if let Err(err) = Environment::check_day(day_format) {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
        if let Err(err) = Environment::check_year(year_format) {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }

    pub fn check_year_dir(year_format: &str) {
        // Verify user is in the correct directory.
        // 1. advent-of-code-{year}
        if let Err(err) = Environment::check_year(year_format) {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}

// tests
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
