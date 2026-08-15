use std::fs;
use std::error::Error;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 { // quick check we dont have too many
        println!{"Too many arguments!\nOnly accepted argument is a single filepath."}
        // to be fair, could just ignore the rest instead of returning.
        // TODO ignore rest of args instead of return
        return;
    }
    // TODO use .get() and handle the optional value
    let filepath = &args[1];
    let message: String = fs::read_to_string(filepath).unwrap();
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
