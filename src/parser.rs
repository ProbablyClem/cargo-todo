extern crate string_parser;
use string_parser::string_parser_with_file;
use colored::Colorize;

pub struct Parser{
    keyword : String,
    end_filter : Box<dyn Fn(Vec<char>) -> bool>,
    callback : Box<dyn Fn(String, usize, &str)>,
}

impl Parser {
    pub fn new(keyword : String, end_filter : Box<dyn Fn(Vec<char>) -> bool>) -> Parser{
        let callback = Box::from(|text : String, line : usize, file : &str| {
            // let path = Path::new(file).strip_prefix(env::current_dir().unwrap().to_str().unwrap()).unwrap();
            println!("{} {} {} {} : {}",file,"TODO".green() ,"Line ".green(), line.to_string().green(), text.blue());
        });
        Parser{keyword: keyword, end_filter : end_filter, callback}
    }

    pub fn new_callback(keyword : String, end_filter : Box<dyn Fn(Vec<char>) -> bool>, callback : Box<dyn Fn(String, usize, &str)>) -> Parser{
        
        Parser{keyword: keyword, end_filter : end_filter, callback}
    }

    fn get_keyword(&self) -> String {
        self.keyword.clone()
    }

    fn get_end_filter(&self) -> &Box<dyn Fn(Vec<char>) -> bool> {
        &self.end_filter
    }

    fn get_callback(&self) -> &Box<dyn Fn(String, usize, &str)> {
        &self.callback
    }

    pub fn parse(&self, path : &str) {
        string_parser_with_file(path, self.get_keyword().as_str(), self.get_end_filter(), self.get_callback()).expect("failed to open file");
    }
}