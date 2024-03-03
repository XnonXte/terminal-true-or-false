use htmlentity::entity::{decode, ICodedDataTrait};
use std::io::{stdin, stdout, Write};

pub fn capitalize(string: &str) -> String {
    let mut chars = string.chars();
    match chars.next() {
        None => String::new(),
        Some(char) => char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn decode_html_entities(content: &str) -> String {
    decode(content.as_bytes()).to_string().unwrap()
}

pub fn input(message: &str) -> String {
    print!("{}", message);
    stdout().flush().expect("Failed to flush stdout");
    let mut tmp = String::new();
    stdin().read_line(&mut tmp).expect("Failed to read line.");

    tmp.trim_end().to_string()
}
