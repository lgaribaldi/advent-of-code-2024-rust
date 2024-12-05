use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod utils;
mod day1;
mod day2;
fn main() {
    let args: Vec<String> = env::args().collect();
    let puzzle = args[1].as_str();
    println!("Running puzzle: {puzzle}");
    let filename = format!("{puzzle}.txt");
    
    if let Ok(lines) = read_lines(filename) {
        match puzzle {
            "1" => day1::run(lines),
            "2" => day2::run(lines),
            _ => println!("{puzzle} not implemented"),
        }
        
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}