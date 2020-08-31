use std::collections::HashMap;
pub struct Parser {
    method: String,
    version: String,
    path: String,
    host: String,
    // header: HashMap<String, String>,
    // body: Strings,
}

impl Parser {
    pub fn new(str: &str) -> Parser {
        let split = str.split("\r\n");
        // println!("{:?}", split.len（）);
        for s in split {
            println!("test:>{}", s)
        }

        Parser {
            method: String::from("GET"),
            version: String::from("1.1"),
            path: String::from("GET"),
            host: String::from("GET"),
            // header: String::from("GET"),
            // header: String::from("GET"),
        }
    }
}
