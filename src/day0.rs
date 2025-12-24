use crate::{Day, parser_utils::{chr, nat, whitespace}};
use parser_combinators::Parser;

pub struct Day0;

impl Day for Day0 {
    type Input = Vec<char>;

    fn day_number() -> i32 {
        2
    }
    
    fn parse_easy<'a>() -> impl Parser<'a, char, O=Self::Input> {
        todo!()
    }

    fn solve_easy(input : Self::Input) -> i32 {
        println!("{input:?}");
        todo!()
    }

    // fn solve_hard(input : Self::Input) -> i32 {
    //     todo!()
    // }
}
