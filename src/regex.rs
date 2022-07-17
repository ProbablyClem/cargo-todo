extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use regex::{Regex};
use crate::token::*;


#[derive(Clone)]
pub struct RegexParser {
    regexes: Vec<Regex>,
    verbosity: i8,
}

impl RegexParser {

    pub fn new(patterns : &Vec<String>, verbosity: i8) -> RegexParser {

        return RegexParser {
            regexes: patterns.iter().map(|p| Regex::new(p).unwrap()).collect(),
            verbosity: verbosity,
        };
    }

    pub fn parse(&self, path : &str) -> Result<Vec<Token>, io::Error>{

        let mut tokens = Vec::new();
        let mut line_cpt = 0;
        for line in read_lines(path)? {

            line_cpt +=1; //The line numbre inside the file so we can display it
            let line = line.unwrap();

            // Loop on regexes
            for regex in &self.regexes {
                if(regex.is_match(&line)){ // If the line matches the regex
                    let matched_string = regex.find(&line).unwrap().as_str(); //Get the part of the string that matched the regex

                    let token = Token::new(path.to_string(), line_cpt,matched_string.to_string(), line.to_string(), self.verbosity);
                    if token.is_some(){
                            tokens.push(token.unwrap());
                        }
                    break;
                }
            }

        
    }
    Ok(tokens)
}
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}

