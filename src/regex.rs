extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::{RegexSet};
use crate::token::*;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn regex_parser(path : &str, regex : Vec<String>) -> Result<Vec<Token>, io::Error>{

    let set = RegexSet::new(regex).unwrap();
    let mut tokens = Vec::new();
    let mut line_cpt = 0;
    for line in read_lines(path)? {
        line_cpt +=1;
        let line = line.unwrap();
        if set.is_match(line.to_lowercase().as_str()){
            tokens.push(Token::new(path.to_string(), line_cpt, line));
            // println!("{}", t);
        }
    }
    Ok(tokens)
}