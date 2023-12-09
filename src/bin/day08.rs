use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

use anyhow::{bail, Result};
use regex::Regex;

mod utils;
use utils::utils::lcm;

#[derive(Debug)]
enum Dirs {
    LEFT,
    RIGHT,
}

impl FromStr for Dirs {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Dirs> {
        use Dirs::*;
        match s {
            "L" => Ok(LEFT),
            "R" => Ok(RIGHT),
            _ => bail!("Bad input: {s}"),
        }
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-08-2023.txt");
    // let input = include_str!("../../inputs/test-08.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn part_one(input: &'static str) -> Result<u64> {
    let mut res = 0;

    let mut lines = input.lines();

    let moves: Vec<Dirs> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Dirs::from_str(c.to_string().deref()).unwrap())
        .collect();

    let mut graph: HashMap<&str, (&str, &str)> = Default::default();
    let pat = Regex::new(r"(?<source>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

    for line in lines.skip(1) {
        let captures = pat.captures(line).unwrap();
        graph.insert(
            captures.name("source").unwrap().as_str(),
            (
                captures.name("left").unwrap().as_str(),
                captures.name("right").unwrap().as_str(),
            ),
        );
    }

    let mut curr = &"AAA";

    loop {
        for dir in &moves {
            res += 1;
            let (left, right) = graph.get(curr).unwrap();
            match dir {
                Dirs::LEFT => curr = left,
                Dirs::RIGHT => curr = right,
            }
            if *curr == "ZZZ" {
                return Ok(res);
            }
        }
    }
}

fn part_two(input: &'static str) -> Result<u64> {
    let mut lines = input.lines();

    let moves: Vec<Dirs> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Dirs::from_str(c.to_string().deref()).unwrap())
        .collect();

    let mut graph: HashMap<&str, (&str, &str)> = Default::default();
    let pat = Regex::new(r"(?<source>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

    for line in lines.skip(1) {
        let captures = pat.captures(line).unwrap();
        graph.insert(
            captures.name("source").unwrap().as_str(),
            (
                captures.name("left").unwrap().as_str(),
                captures.name("right").unwrap().as_str(),
            ),
        );
    }

    let periods = graph.keys().filter(|s| s.ends_with("A")).map(|curr| {
        let mut res = 0;
        let mut curr = curr;
        for dir in moves.iter().cycle() {
            res += 1;
            let (left, right) = graph.get(*curr).unwrap();
            let dest = match dir {
                Dirs::LEFT => left,
                Dirs::RIGHT => right,
            };
            curr = dest;
            if dest.ends_with("Z") { return res; }
        }
        res
    }).collect();

    Ok(lcm(periods))
}
