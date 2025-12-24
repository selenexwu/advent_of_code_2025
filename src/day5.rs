use std::iter::once;

use crate::{Day, parser_utils::{chr, nat}};
use parser_combinators::Parser;

pub struct Day5;

impl Day for Day5 {
    type Input = (Vec<(u64, u64)>, Vec<u64>);

    fn day_number() -> i32 {
        5
    }
    
    fn parse_easy<'a>() -> impl Parser<'a, char, O=Self::Input> {
        ((nat().seql(chr('-')), nat()).sep_end_by1(chr('\n')),
         chr('\n').seqr(nat().sep_end_by1(chr('\n'))))
    }

    fn solve_easy(input : Self::Input) -> u64 {
        // println!("{input:?}");
        let fresh_ranges = &input.0;
        let ids = &input.1;

        let mut count = 0;
        for id in ids {
            for (lo, hi) in fresh_ranges {
                if id >= lo && id <= hi {
                    count += 1;
                    break
                }
            }
        }

        count
    }

    fn solve_hard(input : Self::Input) -> u64 {
        let fresh_ranges = input.0;
        let mut merged_ranges : Vec<(u64, u64)> = Vec::new();

        for new_range @ (new_lo, new_hi) in fresh_ranges {
            let overlapping = merged_ranges.iter().cloned().filter(|(lo, hi)| *hi >= new_lo && new_hi >= *lo);
            let not_overlapping = merged_ranges.iter().cloned().filter(|(lo, hi)| !(*hi >= new_lo && new_hi >= *lo));

            let merged = overlapping.fold(new_range, |(lo1, hi1), (lo2, hi2)| (lo1.min(lo2), hi1.max(hi2)));

            merged_ranges = not_overlapping.chain(once(merged)).collect();
        }
        // println!("{merged_ranges:?}");
        
        merged_ranges.into_iter().map(|(lo, hi)| hi - lo + 1).sum()
    }
}
