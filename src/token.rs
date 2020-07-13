extern crate string_format;
use string_format::string_format;
use std::fmt;
use colored::Colorize;

pub struct Token{
        file : String,
        line : usize,
        keyword : String,
        comment : Option<String>,
        priority : Option<String>,
        date : Option<String>, 
}

impl Token {
    pub fn new (file : String, line : usize, s : String) -> Token{
        println!("{}", s);
        let fields : Vec<&str>= s.split_whitespace().collect();
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
            };
        if fields.len() >= 1 {
            t.keyword = fields[0].to_string();
        }
        if fields.len() >= 2 {
            t.priority = Some(fields[1].to_string());
        }
        if fields.len() >= 3 {
            t.comment = Some(fields[2].to_string());
        }
        if fields.len() >= 4 {
            t.date = Some(fields[3].to_string());
        }

        t
    }

}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Token {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s;
        s = string_format!("{} line: {} {}".to_string(), self.file.clone(), self.line.to_string().green().to_string(), self.keyword.clone().green().to_string());
        if self.priority.is_some(){
            s = string_format!("{}: {}".to_string(), s, self.priority.clone().unwrap());
        }
        if self.comment.is_some() {
            s = string_format!("{} {}".to_string(), s, self.comment.clone().unwrap());
        }
        if self.date.is_some(){
            s = string_format!("{} {}".to_string(), s, self.date.clone().unwrap());
        }
        write!(f, "{}", s)?;
        Ok(())
    }
}