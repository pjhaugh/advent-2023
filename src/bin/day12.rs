use std::collections::HashMap;
use std::num::ParseIntError;
use std::rc::Rc;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::Spring::Unknown;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Damaged),
            '.' => Ok(Self::Operational),
            '?' => Ok(Self::Unknown),
            _ => Err(anyhow!("Bad Value {value}"))
        }
    }
}

fn parse_line(input: &str) -> Result<(Vec<Spring>, Vec<u64>)> {
    let (springs, groups) = input.split_once(" ").expect("Split");
    Ok((springs.chars().map(Spring::try_from).collect::<Result<Vec<Spring>>>()?,
        groups.split(",").map(u64::from_str).collect::<Result<Vec<u64>, ParseIntError>>()?))
}

fn parse_line_two(input: &str) -> Result<(Vec<Spring>, Vec<u64>)> {
    let (springs, groups) = input.split_once(" ").expect("Split");
    let springs = std::iter::repeat(springs).take(5).join("?");
    let groups = std::iter::repeat(groups).take(5).join(",");

    Ok((springs.chars().map(Spring::try_from).collect::<Result<Vec<Spring>>>()?,
        groups.split(",").map(u64::from_str).collect::<Result<Vec<u64>, ParseIntError>>()?))
}


fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-12-2023.txt");
    // let input = include_str!("../../inputs/test-12.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn count(springs: &Vec<Spring>, groups: &Vec<u64>, spring_ptr: usize, group_ptr: usize, group_running: u64) -> u64 {
    if spring_ptr >= springs.len() {
        if group_ptr < groups.len() - 1 {
            return 0;
        } else if group_ptr == groups.len() - 1 && group_running != groups[group_ptr] {
            return 0;
        } else if group_ptr >= groups.len() && group_running > 0 {
            return 0;
        }
        return 1;
    }
    match springs[spring_ptr] {
        Spring::Operational => {
            count_operational(springs, groups, spring_ptr, group_ptr, group_running)
        }
        Spring::Damaged => {
            count(springs, groups, spring_ptr + 1, group_ptr, group_running + 1)
        }
        Unknown => {
            count_operational(springs, groups, spring_ptr, group_ptr, group_running)
                + count(springs, groups, spring_ptr + 1, group_ptr, group_running + 1)
        }
    }
}

fn count_operational(springs: &Vec<Spring>, groups: &Vec<u64>, spring_ptr: usize, group_ptr: usize, group_running: u64) -> u64 {
    if group_running == 0 {
        count(springs, groups, spring_ptr + 1, group_ptr, group_running)
    } else if group_ptr >= groups.len() || group_running != groups[group_ptr] {
        0
    } else {
        count(springs, groups, spring_ptr + 1, group_ptr + 1, 0)
    }
}

fn count_two(springs: &Vec<Spring>, groups: &Vec<u64>, spring_ptr: usize, group_ptr: usize, group_running: u64,
             cache: &mut Rc<HashMap<(usize, usize, u64), u64>>) -> u64 {
    if let Some(res) = cache.get(&(spring_ptr, group_ptr, group_running)){
        return *res;
    }
    if spring_ptr >= springs.len() {
        if group_ptr < groups.len() - 1 {
            return 0;
        } else if group_ptr == groups.len() - 1 && group_running != groups[group_ptr] {
            return 0;
        } else if group_ptr >= groups.len() && group_running > 0 {
            return 0;
        }
        return 1;
    }
    match springs[spring_ptr] {
        Spring::Operational => {
            let res = count_operational_two(springs, groups, spring_ptr, group_ptr, group_running, cache);
            Rc::get_mut(cache).unwrap().insert((spring_ptr, group_ptr, group_running), res);
            res
        }
        Spring::Damaged => {
            let res = count_two(springs, groups, spring_ptr + 1, group_ptr, group_running + 1, cache);
            Rc::get_mut(cache).unwrap().insert((spring_ptr, group_ptr, group_running), res);
            res
        }
        Unknown => {
            let res = count_operational_two(springs, groups, spring_ptr, group_ptr, group_running, cache)
                + count_two(springs, groups, spring_ptr + 1, group_ptr, group_running + 1, cache);
            Rc::get_mut(cache).unwrap().insert((spring_ptr, group_ptr, group_running), res);
            res
        }
    }
}

fn count_operational_two(springs: &Vec<Spring>, groups: &Vec<u64>, spring_ptr: usize, group_ptr: usize, group_running: u64,
                         cache: &mut Rc<HashMap<(usize, usize, u64), u64>>) -> u64 {
    if group_running == 0 {
        count_two(springs, groups, spring_ptr + 1, group_ptr, group_running, cache)
    } else if group_ptr >= groups.len() || group_running != groups[group_ptr] {
        0
    } else {
        count_two(springs, groups, spring_ptr + 1, group_ptr + 1, 0, cache)
    }
}


fn part_one(input: &'static str) -> Result<u64> {
    let lines: Vec<(_, _)> = input.lines().map(|l| parse_line(l).expect("parsing")).collect();
    let mut res = 0;
    for (springs, groups) in &lines {
        let c = count(springs, groups, 0, 0, 0);
        res += c;
    }
    Ok(res)
}


fn part_two(input: &'static str) -> Result<u64> {
    let lines: Vec<(_, _)> = input.lines().map(|l| parse_line_two(l).expect("parsing")).collect();
    let mut res = 0;
    for (springs, groups) in &lines {
        let cache = Default::default();
        let c = count_two(springs, groups, 0, 0, 0, &mut Rc::new(cache));
        res += c;
    }
    Ok(res)
}
