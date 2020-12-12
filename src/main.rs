use std::env;

mod aoc;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (base,extra) = match &args[1].parse::<u32>().unwrap() {
        1 => aoc::day01::solve(&args[2]).unwrap(),
        2 => aoc::day02::solve(&args[2]).unwrap(),
        3 => aoc::day03::solve(&args[2]).unwrap(),
        _ => panic!("Not implemented")
    };

    println!("Base: {}", base);
    println!("Extra: {}", extra);
}

