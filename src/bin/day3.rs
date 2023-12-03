use std::collections::HashSet;

use anyhow::Result;
use nom::Slice;

type Point = (usize, usize);

type NumRegion = (&'static str, (usize, usize), usize); // source, (start, end), y

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-03-2023.txt");
    // let input = include_str!("../../inputs/test-03.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn parse_part_nums(input: &'static str) -> Result<(HashSet<Point>, Vec<NumRegion>)> {
    let mut symbols: HashSet<Point> = Default::default();
    let mut numbers: Vec<NumRegion> = Default::default();
    for (y, line) in input.lines().enumerate() {
        let mut inside_number = false;
        let mut start_number = 0_usize;
        for (x, c) in line.chars().enumerate() {
            if inside_number {
                if !c.is_digit(10) {
                    numbers.push((line, (start_number, x - 1), y));
                    inside_number = false;
                }
            } else {
                if c.is_digit(10) {
                    inside_number = true;
                    start_number = x;
                }
            }
            if !c.is_digit(10) && !c.eq_ignore_ascii_case(&'.') {
                println!("Found {c} at {x}, {y}");
                symbols.insert((x, y));
            }
        }
        if inside_number {
            numbers.push((line, (start_number, line.len() - 1), y));
        }
    }
    Ok((symbols, numbers))
}

fn parse_part_nums_gears(input: &'static str) -> Result<(HashSet<Point>, Vec<NumRegion>)> {
    let mut symbols: HashSet<Point> = Default::default();
    let mut numbers: Vec<NumRegion> = Default::default();
    for (y, line) in input.lines().enumerate() {
        let mut inside_number = false;
        let mut start_number = 0_usize;
        for (x, c) in line.chars().enumerate() {
            if inside_number {
                if !c.is_digit(10) {
                    numbers.push((line, (start_number, x - 1), y));
                    inside_number = false;
                }
            } else {
                if c.is_digit(10) {
                    inside_number = true;
                    start_number = x;
                }
            }
            if c.eq_ignore_ascii_case(&'*') {
                println!("Found {c} at {x}, {y}");
                symbols.insert((x, y));
            }
        }
        if inside_number {
            numbers.push((line, (start_number, line.len() - 1), y));
        }
    }
    Ok((symbols, numbers))
}


fn around(x: usize, y: usize) -> HashSet<Point> {
    let above = y.saturating_sub(1);
    let left = x.saturating_sub(1);
    HashSet::from([
        (left, above), (x, above), (x + 1, above),
        (left, y), (x + 1, y),
        (left, y + 1), (x, y + 1), (x + 1, y + 1),
    ])
}

fn symbol_adjacent(symbols: &HashSet<Point>, start: usize, end: usize, y: usize) -> bool {
    for x in start..=end {
        for point in around(x, y) {
            // println!("Checking {x}, {y}");
            if symbols.contains(&point) {
                println!("Found at {x}, {y}");
                return true;
            }
        }
    }
    false
}

fn part_one(input: &'static str) -> Result<usize> {
    let mut sum = 0;
    let (symbols, num_region) = parse_part_nums(input)?;
    for (line, (start, end), y) in num_region {
        let num = line.slice(start..end + 1).parse::<usize>()?;
        println!("Checking around {num}, {start} to {end} in line {y}");
        if symbol_adjacent(&symbols, start, end, y) {
            sum += num;
        }
    }
    Ok(sum)
}


fn part_two(input: &'static str) -> Result<usize> {
    let mut res = 0;
    let (gears, num_regions) = parse_part_nums_gears(input)?;
    'gear: for (gx, gy) in gears {
        let mut uniq_num = 0;
        let mut ratio: usize = 1;
        let surround = around(gx, gy);
        'number: for (line, (start, end), y) in &num_regions {
            if y.abs_diff(gy) > 1 {
                continue;
            }
            for x in *start..=*end {
                if surround.contains(&(x, *y)) {
                    let num = line.slice(*start..end + 1).parse::<usize>()?;
                    println!("Gear {gx}, {gy} next to {num}");
                    if uniq_num == 2 { continue 'gear; } // This gear has 3+ nums
                    uniq_num += 1;
                    ratio *= num;
                    continue 'number;
                }
            }
        }
        if uniq_num == 2 {
            res += ratio;
        }
    }
    Ok(res)
}
