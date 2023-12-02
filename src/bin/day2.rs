use std::str::FromStr;

use anyhow::{bail, Result};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded};
use nom::IResult;

use crate::Color::{Blue, Green, Red};

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Blue,
    Green,
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "red" => Ok(Red),
            "blue" => Ok(Blue),
            "green" => Ok(Green),
            _ => bail!("Bad color {s}"),
        }
    }
}

type Group = Vec<(u64, Color)>;

#[derive(Debug)]
struct Game {
    id: u64,
    phases: Vec<Group>,
}

fn parse_num(input: &str) -> Result<u64> {
    Ok(u64::from_str_radix(input, 10)?)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = map_res(preceded(tag("Game "), digit1), parse_num)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, phases) = separated_list1(
        tag("; "),
        separated_list1(
            tag(", "),
            pair(
                map_res(digit1, parse_num),
                map_res(preceded(tag(" "), alpha1), Color::from_str),
            ),
        ),
    )(input)?;
    Ok((input, Game { id, phases }))
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-02-2023.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn game_possible(game: &Game, max_red: u64, max_blue: u64, max_green: u64) -> bool {
    for phase in &game.phases {
        let (mut red, mut blue, mut green) = (0, 0, 0);
        for (num, color) in phase {
            match color {
                Red => red += num,
                Blue => blue += num,
                Green => green += num,
            }
        }
        if red > max_red {
            return false;
        }
        if blue > max_blue {
            return false;
        }
        if green > max_green {
            return false;
        }
    }
    true
}

fn part_one(input: &str) -> Result<u64> {
    let (red, blue, green) = (12, 14, 13);

    Ok(input
        .lines()
        .filter_map(|line| {
            let (_, game) = parse_game(line).expect("Bad input");
            if game_possible(&game, red, blue, green) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum())
}

fn calc_power(game: &Game) -> u64 {
    let (mut max_red, mut max_blue, mut max_green) = (0_u64, 0, 0);
    for phase in &game.phases {
        for (num, color) in phase {
            match color {
                Red => max_red = *num.max(&max_red),
                Blue => max_blue = *num.max(&max_blue),
                Green => max_green = *num.max(&max_green),
            }
        }
    }
    max_red * max_green * max_blue
}

fn part_two(input: &str) -> Result<u64> {
    Ok(input
        .lines()
        .map(|line| {
            let (_, game) = parse_game(line).expect("Bad input");
            calc_power(&game)
        })
        .sum())
}
