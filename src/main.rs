use std::fs;
use std::error::Error;

fn main() {
    let message: String = fs::read_to_string("practice.txt").unwrap();
    let mut line_count = 0;
    for line in message.lines() {
        println!("Line: {}", line);
        line_count += 1;
    }
    let mut word_count = 0;
    for word in message.split_whitespace() {
        println!("Word: {}", word);
        word_count += 1;
    }
    println!("Line Count: {}", line_count);
    println!("Word Count: {}", word_count);
    println!("Entire Text: {}", message);
}
