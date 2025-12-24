use crate::{Day, parser_utils::{chr, nat, whitespace}};
use parser_combinators::Parser;

pub struct Day1;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct Rotation {
    dir : Direction,
    amount : i32,
}

impl Day for Day1 {
    type Input = Vec<Rotation>;

    fn day_number() -> i32 {
        1
    }
    
    fn parse_easy<'a>() -> impl Parser<'a, char, O=Self::Input> {
        (chr('L').map_const(Direction::Left).disj(chr('R').map_const(Direction::Right)),
         nat()).map(|(dir, amount)| Rotation { dir, amount }).sep_end_by1(whitespace())
    }

    fn solve_easy(input : Self::Input) -> u64 {
        // println!("{:?}", input);
        let mut dial = 50;
        let mut password = 0;
        for Rotation { dir, amount } in input {
            match dir {
                Direction::Left => dial -= amount,
                Direction::Right => dial += amount,
            }
            dial %= 100;
            if dial == 0 {
                password += 1;
            }
        }
        password
    }

    fn solve_hard(input : Self::Input) -> u64 {
        let mut dial = 50;
        let mut password = 0;
        for Rotation { dir, amount } in input {
            if amount == 0 {
                continue;
            }
            
            let mut new_dial = match dir {
                Direction::Left => dial - amount,
                Direction::Right => dial + amount,
            };

            if dial > 0 && new_dial <= 0 || dial < 0 && new_dial >= 0 {
                password += 1;
            }

            while new_dial >= 100 {
                new_dial -= 100;
                password += 1;
            }

            while new_dial <= -100 {
                new_dial += 100;
                password += 1;
            }

            dial = new_dial;
        }
        password
    }
}
