use std::ops::Add;

use crate::{Day, parser_utils::{chr, nat, whitespace}};
use parser_combinators::Parser;

pub struct Day6;

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Add,
    Mult
}

impl Day for Day6 {
    type Input = ([Vec<u64>; 4], Vec<Op>);

    fn day_number() -> i32 {
        6
    }
    
    fn parse_easy<'a>() -> impl Parser<'a, char, O=Self::Input> {
        (whitespace().seqr(nat()).sep_by1(chr(' ').many1()).sep_end_by1(chr('\n')).map(|v| v.try_into().unwrap()),
         chr('+').map_const(Op::Add).disj(chr('*').map_const(Op::Mult)).sep_by1(chr(' ').many1())).seql(whitespace())
    }

    // Takes advantage of the fact that the input data doesn't already contain any 0s to accept the modified input file
    // Also works on the unmodified input file
    fn solve_easy(input : Self::Input) -> u64 {
        // println!("{input:?}");
        let mut total = 0;
        for ((((n0, n1), n2), n3), op) in input.0[0].iter().zip(input.0[1].iter()).zip(input.0[2].iter()).zip(input.0[3].iter()).zip(input.1.iter()) {
            let mut n0 = *n0;
            while n0 % 10 == 0 {
                n0 /= 10;
            }
            let mut n1 = *n1;
            while n1 % 10 == 0 {
                n1 /= 10;
            }
            let mut n2 = *n2;
            while n2 % 10 == 0 {
                n2 /= 10;
            }
            let mut n3 = *n3;
            while n3 % 10 == 0 {
                n3 /= 10;
            }
            total += match op {
                Op::Add => n0 + n1 + n2 + n3,
                Op::Mult => n0 * n1 * n2 * n3,
            };
        }
        total
    }

    // Requires using a modified input file that replaces spaces inside of number inputs with 0s
    // Takes advantage of the fact that the input data doesn't already contain any 0s
    fn solve_hard(input : Self::Input) -> u64 {
        let mut total = 0;
        for ((((n0, n1), n2), n3), op) in input.0[0].iter().zip(input.0[1].iter()).zip(input.0[2].iter()).zip(input.0[3].iter()).zip(input.1.iter()) {
            let mut solution = match op { Op::Add => 0, Op::Mult => 1 };
            
            let digits = n0.max(n1).max(n2).max(n3).ilog10() + 1;

            for i in 0..digits {
                let d0 = n0 / (10_u64.pow(i)) % 10;
                let d1 = n1 / (10_u64.pow(i)) % 10;
                let d2 = n2 / (10_u64.pow(i)) % 10;
                let d3 = n3 / (10_u64.pow(i)) % 10;
                let mut num = d0 * 1000 + d1 * 100 + d2 * 10 + d3;
                while num % 10 == 0 {
                    num /= 10;
                }
                // println!("{num}");
                match op {
                    Op::Add => solution += num,
                    Op::Mult => solution *= num,
                }
            }

            total += solution;
        }
        total
    }
}
