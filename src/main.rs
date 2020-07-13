extern crate string_format;

extern crate glob;
use std::io::Write;
use std::fs::OpenOptions;
use crate::regex::regex_parser;
use std::path::Path;
use glob::glob;
use std::env;
extern crate walkdir;
extern crate string_parser;
extern crate dirs;
use colored::Colorize;
use std::fs::File;
use std::io::{self, BufRead};
mod parser;
use crate::parser::*;
mod regex;
mod token;

fn main() -> std::io::Result<()> {
    if env::args().last().unwrap() == "--legacy" {
        //this vector containes all the parsers we want to execute
        let mut parsers : Vec<Parser> = vec!();
    
        let mut path = String::from(env::current_dir().unwrap().to_str().unwrap());
        path.push_str("/**/*.rs");
        
        //we add a parser looking for the //todo keyword
        parsers.push(Parser::new(String::from("//todo"), Box::from(|x : Vec<char>| {if  x.last().unwrap() == &'\n' {return true;} else { return false}})));
        //we add a parser looking for the todo!() token
        let _todo_macro_callback = Box::from(|mut text : String, line : usize, file : &str| {
            text.retain(|c| c != '\"');
            let path = Path::new(file).strip_prefix(env::current_dir().unwrap().to_str().unwrap()).unwrap();
            println!("{} {} {} {} : {}",path.to_str().unwrap(),"TODO".green() ,"Line ".green(), line.to_string().green(), text.blue());
        });
        parsers.push(Parser::new_callback(String::from("todo!("), Box::from(|x : Vec<char>| {if  x.last().unwrap() == &')' {return true;} else { return false}}), _todo_macro_callback));

        //support for unimplemented
        let _unimplemented_macro_callback = Box::from(|text : String, line : usize, file : &str| {
            let path = Path::new(file).strip_prefix(env::current_dir().unwrap().to_str().unwrap()).unwrap();
            println!("{} {} {} {} : {}{}{} ",path.to_str().unwrap(),"TODO".green() ,"Line ".green(), line.to_string().green(), "unimplemented!(".blue(), text.magenta(), ")".blue());
        });
        parsers.push(Parser::new_callback(String::from("unimplemented!("), Box::from(|x : Vec<char>| {if  x.last().unwrap() == &')' {return true;} else { return false}}), _unimplemented_macro_callback));

        parsers.push(Parser::new(String::from("//fix"), Box::from(|x : Vec<char>| {if  x.last().unwrap() == &'\n' {return true;} else { return false}})));

        
        //loop on every file within the current dir
        for entry in match glob(&path) {
            Ok(entry) => entry,
            Err(e) => {
                println!("Couldn't access files. Error {}", e);
                Err(e).unwrap()
            }
        } {
            let path = entry.unwrap();
            let path = path.to_str().unwrap();
            
            //execute each parsers on the current file
            for p in &parsers {
                    p.parse(path);
            }
        }
    
     Ok(())

    }
    else{
        let mut path = String::from(dirs::home_dir().unwrap().to_str().unwrap());
        path.push_str("/.cargo/todo_config");
        // println!("{}",path);
        fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
            where P: AsRef<Path>, {
                let file = match File::open(&filename){
                    Ok(line) => line,
                    Err(_) => {
                        println!("{}", "File '~/.cargo/todo_config' not found, creating it".red());
                        let mut f = OpenOptions::new().write(true).read(true).create(true).open("foo.txt")?;
                        f.write_all(b"^s*//s*todo\\b\n")?;
                        f.write_all(b"^s*//s*fix\\b\n")?;
                        f.write_all(b"^s*//s*fixme\\b\n")?;
                        f
                    }
                };
                Ok(io::BufReader::new(file).lines())
        }

        let mut regex = Vec::new();
        for line in read_lines(path)? {
            let line = line.unwrap();
            regex.push(line);
        }

        let mut path = String::from(env::current_dir().unwrap().to_str().unwrap());
        path.push_str("/**/*.rs");

        for entry in match glob(&path) {
            Ok(entry) => entry,
            Err(e) => {
                println!("Couldn't access files. Error {}", e);
                Err(e).unwrap()
            }
        } {
            let path = entry.unwrap();
            let path = path.to_str().unwrap();
            
            //execute each parsers on the current file
            // for p in &parsers {
            //         p.parse(path);
            // }
                regex_parser(path, regex.clone())?;
        }
        Ok(())
}
}

#[allow(dead_code)]
// test zone
//TODO refactor 18-11-2001
//todo implement 18/11/2001 5 getters
//4
//10/10/10
fn test(){
    todo!("implements getters");
}