use crate::{Day, parser_utils::{chr, nat, whitespace}};
use parser_combinators::Parser;

pub struct Day2;

impl Day for Day2 {
    type Input = Vec<(u64, u64)>;

    fn day_number() -> i32 {
        2
    }
    
    fn parse_easy<'a>() -> impl Parser<'a, char, O=Self::Input> {
        (nat().seql(chr('-')), nat()).sep_by1(chr(',')).seql(whitespace())
    }

    fn solve_easy(input : Self::Input) -> u64 {
        // println!("{input:?}");
        let mut total = 0;
        for (start, end) in input {
            for num in start..=end {
                let digits = num.ilog10() + 1;
                if digits % 2 == 1 {
                    continue
                }
                let power = 10_u64.pow(digits / 2);
                if num / power == num % power {
                    total += num;
                }
            }
        }
        total
    }

    fn solve_hard(input : Self::Input) -> u64 {
        // println!("{input:?}");
        let mut total = 0;
        for (start, end) in input {
            for num in start..=end {
                let digits = num.ilog10() + 1;
                let mut valid = true;
                for num_groups in (2..=20).rev() {
                    if !valid {
                        break
                    }
                    if digits % num_groups != 0 {
                        continue
                    }
                    let power = 10_u64.pow(digits / num_groups);
                    let pat = num % power;
                    let mut all_match = true;
                    let mut num = num;
                    for _ in 0..num_groups {
                        if num % power != pat {
                            all_match = false;
                            break;
                        }
                        num /= power;
                    }
                    if all_match {
                        valid = false;
                    }
                }
                if !valid {
                    // println!("{num}");
                    total += num;
                }
            }
        }
        total
    }
}
