extern crate glob;
extern crate walkdir;

use glob::glob;
use std::env;
use std::io::{BufReader, BufRead};
use std::fs::File;
use colored::*;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let mut path = String::from(env::current_dir().unwrap().to_str().unwrap());
    path.push_str("/**/*.rs");

    // regex pattern
    // matches //*todo* case insensitively
    let pattern: Regex = Regex::new("(?i)^\\s*//\\s*todo.*").unwrap();

    // All files in a directory
    for entry in match glob(&path) {
        Ok(entry) => entry,
        Err(e) => {
            println!("Couldn't access files. Error {}", e);
            Err(e).unwrap()
        }
    } {
        let path = entry.unwrap();
        let path = path.to_str().unwrap();
        let reader = BufReader::new(match File::open(path) {
            Ok(f) => f,
            Err(e) => panic!("ERROR!: {}", e),
        });
       
        // All lines in a file
        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if pattern.is_match(&line) {
                // Print out the line if it's a match
                println!("{} line {} :\t{}", path.green(), i + 1, &line);
            }
        }
    }
    Ok(())
}

// Some tests
//todo refactor
// TODO refactor
//TOODrefactor
// TODO                           refactor
