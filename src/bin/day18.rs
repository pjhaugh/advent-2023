use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{bail, Result};
use glam::I64Vec2;
use nom::character::complete::{alphanumeric1, digit1, one_of, space0};
use nom::combinator::map_res;
use nom::sequence::preceded;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-18-2023.txt");
    // let input = include_str!("../../inputs/test-18-2023.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    R,
    L,
    U,
    D,
}

const NORTH: I64Vec2 = I64Vec2::new(0, -1);
const SOUTH: I64Vec2 = I64Vec2::new(0, 1);
const EAST: I64Vec2 = I64Vec2::new(1, 0);
const WEST: I64Vec2 = I64Vec2::new(-1, 0);

impl From<Dir> for Point {
    fn from(value: Dir) -> Self {
        match value {
            Dir::R => EAST,
            Dir::L => WEST,
            Dir::U => NORTH,
            Dir::D => SOUTH,
        }
    }
}

impl Add<Dir> for Point {
    type Output = Point;

    fn add(self, rhs: Dir) -> Self::Output {
        self + Point::from(rhs)
    }
}

type Point = I64Vec2;

#[derive(Debug)]
struct Instruction {
    dir: Dir,
    count: i64,
}

fn parse_inst(input: &'static str) -> Result<Instruction> {
    let (input, dir) = one_of::<_, _, nom::error::Error<_>>("RUDL")(input)?;
    let dir = match dir {
        'U' => Dir::U,
        'D' => Dir::D,
        'L' => Dir::L,
        'R' => Dir::R,
        _ => bail!("Bad dir"),
    };
    let (_, count) = map_res(
        preceded(space0::<_, nom::error::Error<_>>, digit1),
        i64::from_str,
    )(input)?;
    Ok(Instruction { dir, count, })
}

fn parse_inst_hex(input: &'static str) -> Result<Instruction> {
    let input = input.splitn(2, "#").last().unwrap();
    let (_, hex_str) = alphanumeric1::<_, nom::error::Error<_>>(input)?;
    let (length, dir) = hex_str.split_at(5);
    let count = i64::from_str_radix(length, 16)?;
    let dir = match dir {
        "0" => Dir::R,
        "1" => Dir::D,
        "2" => Dir::L,
        "3" => Dir::U,
        _ => bail!("Bad dir"),
    };
    Ok(Instruction { dir, count})
}

fn parse(input: &'static str) -> Result<Vec<Instruction>> {
    input.lines().map(parse_inst).collect::<Result<Vec<_>>>()
}

fn part_one(input: &'static str) -> Result<u64> {
    let instructions = parse(input)?;
    let mut curr = Point::splat(0);
    let mut visited: HashSet<Point> = Default::default();
    for inst in &instructions {
        for _ in 0..inst.count {
            curr = curr + inst.dir;
            visited.insert(curr.clone());
        }
    }
    let mut max_width = 0;
    let mut max_height = 0;
    let mut min_width = i64::MAX;
    let mut min_height = i64::MAX;
    for p in visited.iter() {
        max_width = max(max_width, p.x);
        max_height = max(max_height, p.y);
        min_width = min(min_width, p.x);
        min_height = min(min_height, p.y);
    }

    let mut dug: HashSet<Point> = Default::default();

    for y in min_height..=max_height {
        let mut inside = false;
        for x in min_width - 1..=max_width {
            let in_wall = visited.contains(&Point::new(x, y));
            if in_wall {
                dug.insert(Point::new(x, y));
                if visited.contains(&Point::new(x, y - 1)) {
                    inside = !inside;
                }
            } else {
                if inside {
                    dug.insert(Point::new(x, y));
                }
            }
        }
    }

    for y in min_height - 1..=max_height {
        let mut line: String = Default::default();
        for x in min_width - 1..=max_width {
            line.push(if dug.contains(&Point::new(x, y)) {
                '#'
            } else {
                '.'
            });
        }
    }

    Ok(dug.len() as u64)
}

fn part_two(input: &'static str) -> Result<u64> {
    let instructions = input
        .lines()
        .map(parse_inst_hex)
        .collect::<Result<Vec<_>>>()?;

    let mut sum = 0;

    let mut prev_vertex = Point::splat(0);

    for inst in &instructions {

        let next_vertex = prev_vertex + (Point::from(inst.dir) * inst.count);
        sum += prev_vertex.x * next_vertex.y - prev_vertex.y * next_vertex.x;
        sum += inst.count;
        prev_vertex = next_vertex;
    }

    sum /= 2;

    Ok((sum + 1) as u64)
}
