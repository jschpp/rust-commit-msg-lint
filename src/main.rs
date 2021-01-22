use std::io;
use regex::Regex;

fn first_line(line: &String) {
    let re = Regex::new(r"(fix|feat|build|docs|chore)!?(\(.*?\))?: .+").unwrap();
    assert!(re.is_match(line));
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
        lastline = input;
    }
}
