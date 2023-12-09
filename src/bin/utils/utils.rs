use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::Parser;
use nom::sequence::delimited;
use num::Integer;

//  https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#wrapper-combinators-that-eat-whitespace-before-and-after-a-parser
pub fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
    where
        F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn lcm(vals: Vec<u64>) -> u64 {
    let mut res = 1_u64;
    for val in &vals {
        res = res.lcm(val);
    }
    res
}
