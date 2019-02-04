use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Search for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String
}

fn parse_config(args: &[String]) -> Config {
    // NOTE: args[0] contains the name of the program
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}