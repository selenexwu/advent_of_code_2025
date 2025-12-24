# Rust Parser Combinators

This version is somewhat hacked together based on a Python parser combinator library I wrote for another class project, itself loosely inspired by Parsec. It implements `Parser` as a trait, but almost all parsers are in fact closures (which implement the trait). In my eyes the most interesting development here is the `ParserOnce` trait, which takes `self` instead of `&self` when used to parse something, and therefore can only be used once. It is used internally in places where we need to capture values in a parser that are potentially non-Clone.
