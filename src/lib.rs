use std::{fs::File, io::Read};

use parser_combinators::Parser;
use parser_utils::parse_all;

pub mod parser_utils;
pub mod grid;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub trait Day {
    type Input;

    fn day_number() -> i32;
    
    fn parse_easy<'a>() -> impl Parser<'a, char, O=Self::Input>;
    fn parse_hard<'a>() -> impl Parser<'a, char, O=Self::Input> {
        Self::parse_easy()
    }
    fn solve_easy(input : Self::Input) -> u64;
    fn solve_hard(input : Self::Input) -> u64 {
        Self::solve_easy(input)
    }

    fn run(filename : Option<String>, include_hard : bool) {
        let filename = filename.unwrap_or(format!("inputs/day{}.txt", Self::day_number()));
        let mut raw = String::new();
        File::open(filename).expect("input file exists").read_to_string(&mut raw).expect("input file readable");
        let raw_chars : Vec<char> = raw.chars().collect();

        let easy_parsed = parse_all(Self::parse_easy(), &raw_chars);
        println!("Easy: {}", Self::solve_easy(easy_parsed));

        if include_hard {
            let hard_parsed = parse_all(Self::parse_hard(), &raw_chars);
            println!("Hard: {}", Self::solve_hard(hard_parsed));
        }
    }
    
}
