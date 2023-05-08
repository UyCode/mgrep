use std::error::Error;
use std::{env, fs, process};
use std::collections::{BTreeMap};

/// @author: UyCode \
/// @date: 11:42 05/08/2023 \
/// config class for reading arguments from user input
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        // return error if size of args is less than 2
        // but args is iterator, so we can't get size of it
        // so we use match to get first two arguments
        let size = args.size_hint().0;
        if size < 2 {
            return Err("not enough arguments");
        }

        let user_options = args.next().clone().unwrap();
        let mut is_ignore_case = false;


        if user_options == "-i" {
            is_ignore_case = true;
        }

        if user_options == "-c" {
            let begin_time = std::time::Instant::now();
            let file_path = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file path")
            };
            let count = count_words(&file_path);
            println!("total words count: {}", count?);
            println!("spend time: {:?}", begin_time.elapsed());
            process::exit(0);
        }

        let query;
        if !is_ignore_case {
            query = user_options;
        } else {
            query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get any query string")
            };
        }

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok() || is_ignore_case;
        Ok(Config{query, file_path, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for (index, line) in results {
        println!("{}: {}",index, line.trim());
    }
    //println!("the content is \n{}", contents);

    Ok(())

}
pub fn search<'a>(query: &str, contents: &'a str) -> BTreeMap<usize, &'a str> {

    contents
        .lines()
        .enumerate()
        .map(|(index, line)| (index+1, line))
        .filter(|(_index, line)| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> BTreeMap<usize, &'a str> {

    contents
        .lines()
        .enumerate()
        .map(|(index, line)| (index+1, line))
        .filter(|(_index, line)| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()

}

pub fn count_words(file_path: &str) -> Result<usize, &'static str> {
    let contents = fs::read_to_string(file_path);
    if contents.is_err() {
        return Err("can't read file, please check file path");
    }
    let mut count = 0;
    for word in contents.unwrap().split_whitespace() {
        if word.len() > 0 && word.chars().nth(0).unwrap().is_alphabetic() && (!word.contains(",") || !word.contains(".")) {
            count += 1;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:\n\
        safe, fast, productive.\n\
        pick three.";
        let mut results = BTreeMap::new();
        results.insert(2, "safe, fast, productive.");
        assert_eq!(results, search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:\n\
        safe, fast, productive.\n\
        pick three.";

        let mut results = BTreeMap::new();
        results.insert(2, "safe, fast, productive.");
        assert_eq!(results, search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:\n\
        safe, fast, productive.\n\
        pick three.";
        let mut results = BTreeMap::new();
        results.insert(1, "Rust:");
        assert_eq!(results, search_case_insensitive(query, contents));
    }
}