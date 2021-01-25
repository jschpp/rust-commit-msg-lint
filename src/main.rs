use std::io;
#[macro_use] extern crate lazy_static;
use regex::Regex;

fn first_line(line: &String) {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"^(fix|feat|build|docs|chore)!?(\(.*?\))?: .+").unwrap();
    }
    assert!(RE.is_match(line));
}

fn last_line(line: &String) {
    lazy_static! {
        static ref RE : Regex = Regex::new(r".*").unwrap();
    }
    assert!(RE.is_match(line))
}

fn main() {
    let mut line_count = 0;
    let mut lastline = String::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from pipe");
        input = input.trim().to_string();
        if input == "" {
            break;
        }
        if line_count == 0 {
            first_line(&input);
        }
        println!("Pipe output: {}", input);
        line_count += 1;
        lastline = input.clone();
    }
    last_line(&lastline)
}
