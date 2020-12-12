use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod day01;
pub mod day02;
pub mod day03;

pub type Solution = Result<(String,String),&'static str>;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}