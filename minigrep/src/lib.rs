use std::{error::Error, fs};

use clap::Parser;

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.file_path)?;

    if args.ignore_case {
        search_case_insensitive(&args.query, &contents)
    } else {
        search(&args.query, &contents)
    }
    .iter()
    .for_each(|line| {
        println!("{line}");
    });

    Ok(())
}

#[derive(Parser)]
#[command(about)]
pub struct Args {
    #[arg(index = 1)]
    pub query: String,
    #[arg(index = 2)]
    pub file_path: String,
    #[arg(short, long)]
    pub ignore_case: bool,
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
        ";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
