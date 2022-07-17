extern crate string_format;
extern crate regex;
extern crate chrono;

use chrono::NaiveDate;
use string_format::string_format;
use std::fmt;
use colored::Colorize;
use regex::Regex;
use std::error::Error;

#[derive(Clone)]
pub struct Token{
        file : String,
        line : usize,
        pub keyword : String,
        pub comment : Option<String>,
        pub priority : Option<String>,
        pub date : Option<NaiveDate>,
        pub member : Option<String>, 
        verbosity : i8,
}

impl Token {
    pub fn new (file : String, line : usize, keyword: String, s : String, verbosity : i8) -> Option<Token>{
        // println!("{}", s);

        let content = s.chars().skip(keyword.len()).collect::<String>(); // the content of the line after the keyword
        let fields : Vec<&str>= content.split_whitespace().collect();
        let number_regex = Regex::new("\\b[1-9]\\b").unwrap();
        let date_regex = Regex::new("(\\d*/\\d*/\\d*)").unwrap();
        let member_regex = Regex::new("^!").unwrap();
        if date_regex.is_match("5") {
            panic!("regex");
        }
        // for i in &fields {
        //     println!("{}", i);
        // }

        let mut t = Token {
                file : file,
                line : line,
                keyword: keyword.trim().to_string(),
                comment : None,
                priority : None,
                date : None,
                member : None,
                verbosity : verbosity
            };

        for i in 0..fields.len() {
            if date_regex.is_match(fields[i]){
                t.date = parse_date(fields[i]).unwrap_or_else(|_| {
                         t.add_to_comment(fields[i]); //If we couldn't parse the date, it's part of the comment 
                         None
                    });
            }
            else if member_regex.is_match(fields[i]){
                let mut member = String::new(); //from(fields[i].clone()).chars().next().map(|c| &s[c.len_utf8()..]).unwrap();
                let it = fields[i].chars().skip(1);
                for i in it{
                    member.push(i);
                }

                t.member = Some(member);
            }
            else {
                t.add_to_comment(fields[i]);
            }
        }

        Some(t)
    }

    pub fn inline(&self) {
        let mut s;
        s = string_format!("{} line: {} {} ".to_string(), self.file.clone(), self.line.to_string().green().to_string(), self.keyword.clone().green().to_string());
        if self.member.is_some(){
            s = string_format!("{} Member: {}".to_string(),s ,self.member.clone().unwrap().red().to_string());
        }
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

    fn add_to_comment(&mut self, s : &str) {
        if self.comment.is_none(){
            self.comment = Some(s.to_string());
        }
        else {
            self.comment = Some(string_format!("{} {}".to_string(), self.comment.clone().unwrap(), s.to_string()));
        }
    }
}

fn parse_date(fields : &str) -> Result<Option<NaiveDate>, Box<dyn Error>>{
    let date : Vec<&str> = fields.split("/").collect();
    let y : i32 = date[0].parse::<i32>()?;
    let m : u32 = date[1].parse::<u32>()?;
    let d : u32 = date[2].parse::<u32>()?;
    return Ok(NaiveDate::from_ymd_opt(y, m, d));
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
        if self.member.is_some(){
            s = string_format!("{}Member: {}\n".to_string(),s ,self.member.clone().unwrap().red().to_string());
        }
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
