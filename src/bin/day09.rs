use std::str::FromStr;

use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::{map_res, opt, recognize};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::preceded;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-09-2023.txt");
    // let input = include_str!("../../inputs/test-09.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, l) = separated_list1(
        space1,
        map_res(recognize(preceded(opt(tag("-")), digit1)), i64::from_str),
    )(input)?;
    Ok((input, l))
}

fn compute_sequences(hist: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut res = Vec::new();
    let mut diffs = hist.clone();
    loop {
        diffs = get_diffs(&diffs);
        if diffs.iter().all(|x| x == &0) {
            break;
        }
        res.push(diffs.clone());
    }
    res
}

fn get_diffs(vec: &Vec<i64>) -> Vec<i64> {
    vec.windows(2).map(|x| x[1] - x[0]).collect()
}

fn compute_next(hist: &Vec<i64>) -> i64 {
    let seqs = compute_sequences(hist);
    seqs.iter().map(|v| v.last().unwrap()).sum::<i64>() + hist.last().unwrap()
}

fn compute_prev(hist: &Vec<i64>) -> i64 {
    let seqs = compute_sequences(hist);
    let mut acc = 0;
    for seq in seqs.iter().rev() {
        acc = seq.first().unwrap() - acc;
    }
    hist.first().unwrap() - acc
}

fn part_one(input: &'static str) -> Result<i64> {
    Ok(input
        .lines()
        .map(|l| {
            let (_, v) = parse_line(l).expect("Parser");
            v
        })
        .map(|h| compute_next(&h))
        .sum())
}

fn part_two(input: &'static str) -> Result<i64> {
    Ok(input
        .lines()
        .map(|l| {
            let (_, v) = parse_line(l).expect("Parser");
            v
        })
        .map(|h| compute_prev(&h))
        .sum())
}
