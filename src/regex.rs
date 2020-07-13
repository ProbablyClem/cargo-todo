use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate regex;
use regex::{RegexSet};
use std::env;
use colored::Colorize;
use super::*;
use crate::token::*;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn regex_parser(path : &str, regex : Vec<String>) -> Result<(), io::Error>{

    // let re = Regex::new(r"//s*todo\b").unwrap();
    let set = RegexSet::new(regex).unwrap();

    let mut line_cpt = 0;
    let path = Path::new(path).strip_prefix(env::current_dir().unwrap().to_str().unwrap()).unwrap();
    for line in read_lines(path)? {
        line_cpt +=1;
        let line = line.unwrap();
        if set.is_match(line.to_lowercase().as_str()){
            // println!("{} {}: {}",path.to_str().unwrap(), line_cpt.to_string().green(), line);
            let t = Token::new(path.to_str().unwrap().to_string(), line_cpt, line);
            // t.print();
            println!("{}", t);
        }
    }
    Ok(())
}