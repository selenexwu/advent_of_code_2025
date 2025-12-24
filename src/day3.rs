use crate::{Day, parser_utils::{chr, digit}};
use parser_combinators::Parser;

pub struct Day3;

impl Day for Day3 {
    type Input = Vec<Vec<u8>>;

    fn day_number() -> i32 {
        3
    }
    
    fn parse_easy<'a>() -> impl Parser<'a, char, O=Self::Input> {
        digit().map(|d| d.to_digit(10).unwrap() as u8).many1().sep_end_by1(chr('\n'))
    }

    fn solve_easy(input : Self::Input) -> u64 {
        // println!("{input:?}");
        let mut total : u64 = 0;
        for line in input {
            let mut most = 0;
            let mut second_most = 0;
            let len = line.len();
            for (i, jolt) in line.into_iter().enumerate() {
                if jolt > most && i != len - 1{
                    most = jolt;
                    second_most = 0;
                } else if jolt > second_most {
                    second_most = jolt;
                }
            }
            // println!("{most}, {second_most}");
            total += (most * 10 + second_most) as u64;
        }
        total
    }

    fn solve_hard(input : Self::Input) -> u64 {
        // println!("{input:?}");
        let mut total : u64 = 0;
        for line in input {
            let mut mosts = [0; 12];
            let len = line.len();
            for (i, jolt) in line.into_iter().enumerate() {
                for j in 0..12 {
                    if jolt > mosts[j] && i <= len - 12 + j {
                        mosts[j] = jolt;
                        for k in j+1..12 {
                            mosts[k] = 0;
                        }
                        break
                    }
                }
            }
            // println!("{mosts:?}");
            let mut val : u64 = 0;
            for digit in mosts {
                val *= 10;
                val += digit as u64;
            }
            total += val;
        }
        total
    }
}
