use std::cmp::min;
use std::collections::VecDeque;
use std::str::FromStr;

use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::error::ErrorKind;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::Parser;

#[derive(Debug)]
struct Mapping {
    dest_start: u64,
    source_start: u64,
    len: u64,
}

struct Map {
    mappings: Vec<Mapping>,
    name: &'static str,
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-05-2023.txt");
    // let input = include_str!("../../inputs/test-05.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn convert(map: &Map, val: &u64) -> u64 {
    for mapping in &map.mappings {
        if (mapping.source_start..mapping.source_start + mapping.len).contains(val) {
            return mapping.dest_start + (val - mapping.source_start);
        }
    }
    *val
}

fn convert_split(map: &Map, range: &Range) -> Vec<Range> {
    println!("Processing {range:?} using {}", map.name);
    let mut res: Vec<Range> = Default::default();
    let mut remaining = Range {
        start: range.start,
        len: range.len,
    };
    'next: while remaining.len > 0 {
        let mut next_overlapping: Option<&Mapping> = None;
        for mapping in &map.mappings {
            if (mapping.source_start..mapping.source_start + mapping.len).contains(&remaining.start)
            {
                let dist_to_start = remaining.start - mapping.source_start;
                let mappable_len = mapping.len - dist_to_start;
                let mapped_len = min(mappable_len, remaining.len);
                let sub_map = Range {
                    start: mapping.dest_start + dist_to_start,
                    len: mapped_len,
                };
                res.push(sub_map);
                let new_start = remaining.start + mapped_len;
                let new_len = remaining.len.saturating_sub(mapped_len);
                println!("Mapped {remaining:?} to {sub_map:?} based on {mapping:?}. Remaining: {new_start} {new_len}");
                remaining = Range {
                    start: new_start,
                    len: new_len,
                };
                continue 'next;
            } else {
                if (remaining.start..(remaining.start + remaining.len)).contains(&mapping.source_start) {
                    next_overlapping = match next_overlapping {
                        None => Some(&mapping),
                        Some(other) => {
                            if mapping.source_start < other.source_start {
                                Some(&mapping)
                            } else {
                                Some(other)
                            }
                        }
                    }
                }
            }
        }
        match next_overlapping {
            None => {
                println!("No overlap, mapping {remaining:?} to itself");
                res.push(remaining);
                break;
            }
            Some(mapping) => {
                println!("Mapping {remaining:?} to two ranges:");
                let not_overlapped = Range {
                    start: remaining.start,
                    len: mapping.source_start - remaining.start,
                };
                res.push(not_overlapped);
                remaining = Range {
                    start: mapping.source_start,
                    len: remaining.len - (mapping.source_start - remaining.start),
                };
                println!("\t{not_overlapped:?}");
                println!("\t{remaining:?}");
            }
        }
    }
    res
}

fn part_one(input: &'static str) -> Result<u64> {
    let mut sections = input.split("\n\n");

    let mut seeds = sections
        .next()
        .unwrap()
        .split_at(7)
        .1
        .split_ascii_whitespace()
        .map(|s| u64::from_str(s).unwrap())
        .collect::<Vec<_>>();

    // println!("{seeds:?}");

    let maps = sections
        .map(|m| {
            let mut lines = m.lines();
            let name = lines.next().unwrap();
            let mappings = lines
                .map(|l| {
                    let nums = l
                        .split_ascii_whitespace()
                        .map(|s| u64::from_str(s).unwrap())
                        .collect::<Vec<u64>>();
                    Mapping {
                        dest_start: nums[0],
                        source_start: nums[1],
                        len: nums[2],
                    }
                })
                .collect::<Vec<_>>();
            Map { mappings, name }
        })
        .collect::<Vec<_>>();

    // println!("{maps:?}");

    // let mut res = seeds.clone();

    for map in &maps {
        for index in 0..seeds.len() {
            let nex_val = convert(map, seeds.get(index).expect("Index out of bounds"));
            seeds[index] = nex_val;
        }
    }

    // println!("{seeds:?}");

    Ok(*seeds.iter().min().expect("shouldn't be empty"))
}

#[derive(Debug, Copy, Clone)]
struct Range {
    start: u64,
    len: u64,
}

fn parse(input: &'static str) -> Result<Vec<Range>> {
    let (_, res) = preceded(
        tag("seeds: "),
        separated_list1(
            space1::<&str, (_, ErrorKind)>,
            separated_pair(
                digit1.map(|s| u64::from_str(s).unwrap()),
                space1,
                digit1.map(|s| u64::from_str(s).unwrap()),
            )
            .map(|(start, len)| Range { start, len }),
        ),
    )(input)?;
    Ok(res)
}

fn part_two(input: &'static str) -> Result<u64> {
    let mut sections = input.split("\n\n");

    let mut seeds = VecDeque::from(parse(sections.next().unwrap())?);

    println!("{seeds:?}");

    let maps = sections
        .map(|m| {
            let mut lines = m.lines();
            let name = lines.next().unwrap();
            let mappings = lines
                .map(|l| {
                    let nums = l
                        .split_ascii_whitespace()
                        .map(|s| u64::from_str(s).unwrap())
                        .collect::<Vec<u64>>();
                    Mapping {
                        dest_start: nums[0],
                        source_start: nums[1],
                        len: nums[2],
                    }
                })
                .collect::<Vec<_>>();
            Map { mappings, name }
        })
        .collect::<Vec<_>>();

    for map in &maps {
        for _ in 0..seeds.len() {
            let next_vals = convert_split(map, &seeds.pop_front().unwrap());
            next_vals.iter().for_each(|sr| seeds.push_back(*sr));
        }
    }

    println!("{seeds:?}");

    Ok(seeds.iter().map(|sr| sr.start).min().unwrap())
}
