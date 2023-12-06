use anyhow::Result;
use std::iter::zip;
use std::str::FromStr;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-06-2023.txt");
    // let input = include_str!("../../inputs/test-06.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn part_one(input: &'static str) -> Result<u64> {
    let mut res = 1;

    let mut lines = input.lines();

    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .split_at(6)
        .1
        .trim()
        .split_whitespace()
        .map(|s| u64::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .split_at(10)
        .1
        .trim()
        .split_whitespace()
        .map(|s| u64::from_str(s).unwrap())
        .collect::<Vec<_>>();

    println!("{times:?}");
    println!("{distances:?}");

    for (time, distance) in zip(times, distances) {
        res *= (0..time).filter(|t| t * (time - t) > distance).count() as u64;
    }

    Ok(res)
}

fn part_two(input: &'static str) -> Result<u64> {
    let mut lines = input.lines();

    let time: u64 = lines
        .next()
        .unwrap()
        .split_at(6)
        .1
        .replace(" ", "")
        .parse()?;
    let distance: u64 = lines
        .next()
        .unwrap()
        .split_at(10)
        .1
        .replace(" ", "")
        .parse()?;

    Ok((0..time).filter(|t| t * (time - t) > distance).count() as u64)
}
