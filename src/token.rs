extern crate string_format;
extern crate regex;
extern crate chrono;

use chrono::NaiveDate;
use string_format::string_format;
use std::fmt;
use colored::Colorize;
use regex::Regex;

#[derive(Clone)]
pub struct Token{
        file : String,
        line : usize,
        pub keyword : String,
        pub comment : Option<String>,
        pub priority : Option<String>,
        pub date : Option<NaiveDate>, 
        verbosity : i8,
}

impl Token {
    pub fn new (file : String, line : usize, s : String, verbosity : i8) -> Token{
        // println!("{}", s);
        let fields : Vec<&str>= s.split_whitespace().collect();
        let number_regex = Regex::new("\\b[1-9]\\b").unwrap();
        let date_regex = Regex::new("(\\d*/\\d*/\\d*)").unwrap();
        if date_regex.is_match("5") {
            panic!("regex");
        }
        // for i in &fields {
        //     println!("{}", i);
        // }

        let mut t = Token {
                file : file,
                line : line,
                keyword: "todo".to_string(),
                comment : None,
                priority : None,
                date : None,
                verbosity : verbosity
            };

        for i in 0..fields.len() {
            if i == 0{
                t.keyword = fields[0].to_string().to_lowercase();
            }
            else if number_regex.is_match(fields[i]) {
                t.priority = Some(fields[i].to_string());
            }
            else if date_regex.is_match(fields[i]){
                let date : Vec<&str> = fields[i].split("/").collect();
                t.date = NaiveDate::from_ymd_opt(date[0].parse::<i32>().unwrap(), date[1].parse::<u32>().unwrap(), date[2].parse::<u32>().unwrap());
                // t.date = Some(fields[i].to_string());
            }
            else {
                if t.comment.is_none(){
                    t.comment = Some(fields[i].to_string());
                }
                else{
                t.comment = Some(string_format!("{} {}".to_string(),t.comment.unwrap(), fields[i].to_string()));
                }
            }
        }

        t
    }

    pub fn inline(&self) {
        let mut s;
        s = string_format!("{} line: {} {} ".to_string(), self.file.clone(), self.line.to_string().green().to_string(), self.keyword.clone().green().to_string());
        if self.priority.is_some(){
            s = string_format!("{} Priority: {}".to_string(), s, self.priority.clone().unwrap().red().to_string());
        }
        if self.date.is_some(){
            s = string_format!("{} Deadline: {}".to_string(), s, self.date.clone().unwrap().to_string().red().to_string());
        }
        if self.comment.is_some() {
            s = string_format!("{} {}".to_string(), s, self.comment.clone().unwrap().blue().to_string());
        }
        println!("{}", s);
    }

}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Token {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s;

        s = string_format!("{} line: {} {} \n".to_string(), self.file.clone(), self.line.to_string().green().to_string(), self.keyword.clone().green().to_string());
        if self.verbosity <= 1{
            if self.comment.is_some() {
                s = string_format!("{}{}\n".to_string(), s, self.comment.clone().unwrap().blue().to_string());
            }
        }
        else {
        if self.priority.is_some(){
            s = string_format!("{}Priority: {}\n".to_string(), s, self.priority.clone().unwrap().red().to_string());
        }
        if self.date.is_some(){
            s = string_format!("{}Deadline: {}\n".to_string(), s, self.date.clone().unwrap().to_string().red().to_string());
        }
        if self.comment.is_some() {
            s = string_format!("{}{}\n".to_string(), s, self.comment.clone().unwrap().blue().to_string());
        }
    }
        
        write!(f, "{}", s)?;
        Ok(())
    }
}
