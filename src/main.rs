use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&args[1]) {

        let values: Vec<u32> = lines.map(|s| s.unwrap().parse::<u32>().unwrap() ).collect();

        // Consumes the iterator, returns an (Optional) String
        for val1 in &values {
            for val2 in &values {
                if 2020 == val1 + val2 {
                    println!("{}", val1*val2);
                }
            }
        }

        // Consumes the iterator, returns an (Optional) String
        for val1 in &values {
            for val2 in &values {
                for val3 in &values {
                    if 2020 == val1 + val2 + val3 {
                            println!("{}", val1*val2*val3);
                    }
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
