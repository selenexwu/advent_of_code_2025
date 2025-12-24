use std::env::args;

use aoc_2025::{Day, day1::Day1, day2::Day2};

fn main() {
    let args : Vec<_> = args().skip(1).take(3).collect();

    let day_number : usize = args[0].parse().expect("first argument is day number");
    
    let include_hard = args.len() >= 2;

    let mut filename : Option<String> = None;
    if args.len() >= 3 {
        filename = Some(args[2].clone());
    }

    let days = [Day1::run, Day2::run];
    days[day_number - 1](filename, include_hard);
}
