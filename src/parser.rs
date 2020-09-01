use std::collections::HashMap;

#[derive(Debug)]
pub struct Parser {
    method: String,
    version: String,
    path: String,
    header: HashMap<String, String>,
    // body: Strings,
}

impl Parser {
    pub fn new(str: &str) -> Parser {
        let split: Vec<&str> = str.split("\r\n").collect();

        let mut first_vec: Vec<&str> = Vec::new();
        let mut header: HashMap<String, String> = HashMap::new();
        for (index, s) in split.iter().enumerate() {
            if index == 0 {
                first_vec = s.split(" ").collect();
                continue;
            }
            let item_split: Vec<&str> = s.split(": ").collect();
            if let [name, value] = item_split[..] {
                header.insert(String::from(name), String::from(value));
            }
        }

        Parser {
            method: String::from(first_vec[0]),
            version: String::from(first_vec[2]),
            path: String::from(first_vec[1]),
            header,
        }
    }
}
