use std::fmt::Write;
use std::str::FromStr;

use anyhow::{bail, Result};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::map_res;
use nom::multi::{many0, many_till};
use nom::IResult;

fn parse_digit(input: &str) -> Result<u32> {
    match input {
        "0" => Ok(0),
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        "4" => Ok(4),
        "5" => Ok(5),
        "6" => Ok(6),
        "7" => Ok(7),
        "8" => Ok(8),
        "9" => Ok(9),
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        "zero" => Ok(0),
        _ => bail!("Bad num {input}"),
    }
}
fn parse_digit_backwards(input: &str) -> Result<u32> {
    match input {
        "0" => Ok(0),
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        "4" => Ok(4),
        "5" => Ok(5),
        "6" => Ok(6),
        "7" => Ok(7),
        "8" => Ok(8),
        "9" => Ok(9),
        "eno" => Ok(1),
        "owt" => Ok(2),
        "eerht" => Ok(3),
        "ruof" => Ok(4),
        "evif" => Ok(5),
        "xis" => Ok(6),
        "neves" => Ok(7),
        "thgie" => Ok(8),
        "enin" => Ok(9),
        "orez" => Ok(0),
        _ => bail!("Bad num {input}"),
    }
}

fn parse_num(input: &str) -> IResult<&str, u32> {
    let (input, (_, res)) = many_till(
        anychar,
        map_res(
            alt((
                tag("1"),
                tag("2"),
                tag("3"),
                tag("4"),
                tag("5"),
                tag("6"),
                tag("7"),
                tag("8"),
                tag("9"),
                tag("one"),
                tag("two"),
                tag("three"),
                tag("four"),
                tag("five"),
                tag("six"),
                tag("seven"),
                tag("eight"),
                tag("nine"),
                tag("zero"),
                tag("0"),
            )),
            parse_digit,
        ),
    )(input)?;

    Ok((input, res))
}

fn parse_num_backwards(input: &str) -> IResult<&str, u32> {
    let (input, (_, res)) = many_till(
        anychar,
        map_res(
            alt((
                tag("1"),
                tag("2"),
                tag("3"),
                tag("4"),
                tag("5"),
                tag("6"),
                tag("7"),
                tag("8"),
                tag("9"),
                tag("eno"),
                tag("owt"),
                tag("eerht"),
                tag("ruof"),
                tag("evif"),
                tag("xis"),
                tag("neves"),
                tag("thgie"),
                tag("enin"),
                tag("orez"),
                tag("0"),
            )),
            parse_digit_backwards,
        ),
    )(input)?;

    Ok((input, res))
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-01-2023.txt");
    let part_one_ans = part_one(input)?;
    println!("Part one: {part_one_ans}");

    let part_two_ans = part_two(input)?;
    println!("Part two: {part_two_ans}");

    Ok(())
}

fn part_one(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(|s| {
            let mut o: String = Default::default();
            o.write_char(s.chars().filter(char::is_ascii_digit).next().unwrap())
                .unwrap();
            o.write_char(s.chars().rev().filter(char::is_ascii_digit).next().unwrap())
                .unwrap();
            u32::from_str(o.as_str()).unwrap()
        })
        .sum::<u32>())
}

fn part_two(input: &str) -> Result<u32> {
    let mut x = 0;
    for line in input.lines() {
        let (_, tens) = many0(parse_num)(line).unwrap();
        let rev = line.to_string().chars().rev().collect::<String>();
        let y = rev.as_str();
        let (_, ones) = many0(parse_num_backwards)(y).unwrap();
        let v = tens.first().unwrap() * 10 + ones.first().unwrap();
        x += v;
    }
    Ok(x)
}
