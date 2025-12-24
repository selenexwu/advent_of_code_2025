use crate::{Day, parser_utils::chr, grid::Grid};
use parser_combinators::Parser;

pub struct Day4;

impl Day for Day4 {
    type Input = Grid<bool>;

    fn day_number() -> i32 {
        4
    }
    
    fn parse_easy<'a>() -> impl Parser<'a, char, O=Self::Input> {
        chr('.').map_const(false).disj(chr('@').map_const(true)).many1().sep_end_by1(chr('\n')).map(Grid::new)
    }

    fn solve_easy(input : Self::Input) -> u64 {
        // println!("{input:?}");
        input.index_iter().filter(|p| input[*p] && input.neighbors8(*p).filter(|x| **x).count() < 4).count() as u64
    }

    fn solve_hard(mut input : Self::Input) -> u64 {
        let mut total = 0;
        loop {
            let accessible = input.index_iter().filter(|p| input[*p] && input.neighbors8(*p).filter(|x| **x).count() < 4).collect::<Vec<_>>();
            if accessible.len() == 0 {
                break
            }
            for p in accessible {
                total += 1;
                input[p] = false;
            }
        }
        total
    }
}
