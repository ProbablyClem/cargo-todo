extern crate glob;

use std::path::{Path, PathBuf};
use glob::glob;
use std::env;
extern crate walkdir;
extern crate string_parser;
use string_parser::string_parser_with_file;
use colored::*;

fn main() -> std::io::Result<()> {
    let mut path = String::from(env::current_dir().unwrap().to_str().unwrap());
    path.push_str("/**/*.rs");
    fn todo_comment_end_filter(c : Vec<char>) -> bool{
        if c.last().unwrap() == &'\n' {
            return true;
        }
        else {
            return false;
        }
    }

    fn todo_comment_callback(s : String, l : usize, p : &str){
        let path = Path::new(p).strip_prefix(env::current_dir().unwrap().to_str().unwrap()).unwrap();
        println!("{} {} {} {} : {}",path.to_str().unwrap(),"TODO".green() ,"Line ".green(), l.to_string().green(), s.blue());
    }

    fn todo_macro_end_filter(c : Vec<char>) -> bool{
        if c.last().unwrap() == &')' {
            return true;
        }
        else {
            return false;
        }
    }

    fn todo_macro_callback(s : String, l : usize, p : &str){
        let path = Path::new(p).strip_prefix(env::current_dir().unwrap().to_str().unwrap()).unwrap();
        println!("{} {} {} : {}",path.to_str().unwrap(), "Line ".green(), l.to_string().green(), s.blue());
    }

    for entry in match glob(&path) {
        Ok(entry) => entry,
        Err(e) => {
            println!("Couldn't access files. Error {}", e);
            Err(e).unwrap()
        }
    } {
        let path = entry.unwrap();
        let path = path.to_str().unwrap();
        string_parser_with_file(path.clone(), "//todo", todo_comment_end_filter, todo_comment_callback).expect("failed to open file");
        string_parser_with_file(path.clone(), "todo!(", todo_macro_end_filter, todo_comment_callback).expect("failed to open file");

    }
    Ok(())
    
}

//todo refactor
fn test(){
    todo!("hey");
}