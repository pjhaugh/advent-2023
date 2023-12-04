use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::Parser;
use nom::sequence::delimited;

//  https://github.com/rust-bakery/nom/blob/main/doc/nom_recipes.md#wrapper-combinators-that-eat-whitespace-before-and-after-a-parser
pub fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
    where
        F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}