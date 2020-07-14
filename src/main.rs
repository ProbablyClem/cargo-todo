extern crate string_format;
extern crate clap;
extern crate walkdir;
extern crate string_parser;
extern crate dirs;
extern crate glob;

use glob::glob;
use colored::Colorize;
use clap::{Arg, App, SubCommand};

//local files
mod parser;
mod regex;
mod token;
use crate::parser::*;
use crate::regex::regex_parser;
use crate::token::Token;

//std
use std::io::Write;
use std::fs::OpenOptions;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};




fn main() -> std::io::Result<()> {

    let matches = App::new("Cargo-todo")
                          .author("Clément Guiton <clement.guiton.dev@gmail.com>")
                          .about("cargo tool to find TODOs in your code")
                          .arg(Arg::with_name("inline")
                               .short("i")
                               .long("inline")
                               .value_name("inline")
                               .help("display todos in one line")
                               .takes_value(false))
                          .arg(Arg::with_name("filter")
                                .help("filter todos to show")
                                .short("f")
                                .long("filter")
                                .takes_value(true))
                          .arg(Arg::with_name("verbose")
                               .short("v")
                               .long("verbose")
                               .multiple(true)
                               .help("Sets the level of verbosity"))
                          .subcommand(SubCommand::with_name("legacy")
                                .about("launch program in legacy mode (supports todo!(), etc..."))
                          .get_matches();

    match matches.occurrences_of("v") {
    0 => println!("Some verbose info"),
    1 => println!("less verbose info"),
    2 => println!("Some verbose info"),
    3 | _ => println!("you already see everything"),
    }

    if let Some(_matches) = matches.subcommand_matches("legacy") {
        let mut parsers : Vec<Parser> = vec!();
    
        let mut path = String::from(env::current_dir().unwrap().to_str().unwrap());
        path.push_str("/**/*.rs");
        
        //we add a parser looking for the //todo keyword
        parsers.push(Parser::new(String::from("//todo"), Box::from(|x : Vec<char>| {if  x.last().unwrap() == &'\n' {return true;} else { return false}})));
        //we add a parser looking for the todo!() token
        let _todo_macro_callback = Box::from(|mut text : String, line : usize, file : &str| {
            text.retain(|c| c != '\"');
            println!("{} {} {} {} : {}",file,"TODO".green() ,"Line ".green(), line.to_string().green(), text.blue());
        });
        parsers.push(Parser::new_callback(String::from("todo!("), Box::from(|x : Vec<char>| {if  x.last().unwrap() == &')' {return true;} else { return false}}), _todo_macro_callback));

        //support for unimplemented
        let _unimplemented_macro_callback = Box::from(|text : String, line : usize, file : &str| {
            println!("{} {} {} {} : {}{}{} ",file,"TODO".green() ,"Line ".green(), line.to_string().green(), "unimplemented!(".blue(), text.magenta(), ")".blue());
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
            let path = Path::new(&path).strip_prefix(env::current_dir().unwrap().to_str().unwrap()).unwrap();
            if !path.starts_with("target/"){
                let path = path.to_str().unwrap();
            //execute each parsers on the current file
            for p in &parsers {
                p.parse(path);
                }
            }

            
        }
    
     Ok(())
    }
    else{
        let mut tokens : Vec<Token> = Vec::new();

        let mut path = String::from(dirs::home_dir().unwrap().to_str().unwrap());
        path.push_str("/.cargo/todo_config");
        // println!("{}",path);
        fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
            where P: AsRef<Path>, {
                let file = match File::open(&filename){
                    Ok(line) => line,
                    Err(_) => {
                        println!("{}", "File '~/.cargo/todo_config' not found, creating it".red());
                        let mut f = OpenOptions::new().write(true).read(true).create(true).open(&filename).unwrap();
                        f.write_all(b"^s*//s*todo\\b\n").unwrap();
                        f.write_all(b"^s*//s*fix\\b\n").unwrap();
                        f.write_all(b"^s*//s*fixme\\b\n").unwrap();
                        return read_lines(filename);
                    }
                };
                Ok(io::BufReader::new(file).lines())
        }

        let mut regex = Vec::new();
        for line in read_lines(path).unwrap() {
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
            let path = Path::new(&path).strip_prefix(env::current_dir().unwrap().to_str().unwrap()).unwrap();
            // println!("{}", path.to_str().unwrap());
            if !path.starts_with("target/"){
                let path = path.to_str().unwrap();
            
                
                match regex_parser(path, regex.clone()){
                    Ok(mut t) => {
                        tokens.append(&mut t);
                    },
                    Err(e) => eprintln!{"{}", e},
                }
            }
            
        }
        if matches.is_present("inline"){
            for i in tokens{
                i.inline();
            }
        }
        else {
            for i in tokens {
                println!("{}", i);
            }
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