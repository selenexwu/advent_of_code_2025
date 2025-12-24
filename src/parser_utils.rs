use std::{fmt::Debug, str::FromStr};

use parser_combinators::*;

pub fn chr<'a>(c: char) -> impl Parser<'a, char, O = &'a char> {
    read().satisfy(move |c1| **c1 == c)
}

pub fn keyword<'a>(kw: &str) -> impl Parser<'a, char, O = ()> {
    let kw: Vec<char> = kw.chars().collect();
    move |tokens: &'a [char]| {
        if tokens.starts_with(&kw) {
            Ok(((), &tokens[kw.len()..]))
        } else {
            Err(ParseError {
                typ: ErrorType::Msg("keyword".to_string()),
                rest: tokens,
            })
        }
    }
}

pub fn whitespace<'a>() -> impl Parser<'a, char, O = ()> {
    read()
        .satisfy(|c: &&char| c.is_whitespace())
        .many()
        .map_const(())
}

pub fn digit<'a>() -> impl Parser<'a, char, O = &'a char> {
    read().satisfy(|c: &&char| c.is_digit(10))
}

pub fn nat<'a, N>() -> impl Parser<'a, char, O = N>
where
    N : FromStr,
    <N as FromStr>::Err: Debug,
{
    digit().many1().map(|ds| ds.into_iter().collect::<String>().parse().expect("digits parse to int"))
}

pub fn parse_all<'a, O, P : Parser<'a, char, O=O>>(parser : P, input : &'a [char]) -> O {
    let (res, rest) = parser.parse(input).expect("parse succeeds");
    if !rest.is_empty() {
        println!("{rest:?}");
        panic!("parse should handle all input");
    }
    res
}
