extern crate glob;

use glob::glob;
use std::env;
extern crate walkdir;
extern crate string_parser;
use string_parser::string_parser_with_line;
use colored::*;

fn main() -> std::io::Result<()> {
    let mut path = String::from(env::current_dir().unwrap().to_str().unwrap());
    path.push_str("/**/*.rs");

    fn end_filter(c : Vec<char>) -> bool{
        c.last().unwrap() == &'\n'
    }

    fn callback(s : String, l : usize){
        println!("{} {} : {}","Line ".green(), l.to_string().green(), s.blue());
    }

    for entry in match glob(&path) {
        Ok(entry) => entry,
        Err(e) => {
            println!("Couldn't access files. Error {}", e);
            Err(e).unwrap()
        }
    } {
        string_parser_with_line(entry.unwrap().to_str().unwrap(), "//todo", end_filter, callback).expect("failed to open file");
    }
    Ok(())
}

//todo refactor
