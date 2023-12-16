use std::cmp::max;
use std::collections::{HashMap, HashSet};

use anyhow::Result;
use glam::IVec2;
use itertools::Itertools;

use crate::Object::{Mirror, Splitter};

#[derive(Debug, Copy, Clone)]
enum Splitters {
    Horizontal,
    // "-"
    Vertical, // "|"
}

#[derive(Debug, Copy, Clone)]
enum Mirrors {
    Left,
    // "\"
    Right, // "/"
}

#[derive(Debug, Copy, Clone)]
enum Object {
    Splitter(Splitters),
    Mirror(Mirrors),
}

type Point = IVec2;

#[derive(Debug)]
struct Map {
    objects: HashMap<Point, Object>,
    width: i32,
    height: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

const NORTH: IVec2 = IVec2::new(0, -1);
const SOUTH: IVec2 = IVec2::new(0, 1);
const EAST: IVec2 = IVec2::new(1, 0);
const WEST: IVec2 = IVec2::new(-1, 0);

impl Into<Point> for Dir {
    fn into(self) -> IVec2 {
        match self {
            Dir::North => NORTH,
            Dir::South => SOUTH,
            Dir::East => EAST,
            Dir::West => WEST,
        }
    }
}

fn parse(input: &str) -> Map {
    let objects = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                if let Some(o) = match c {
                    '-' => Some(Splitter(Splitters::Horizontal)),
                    '|' => Some(Splitter(Splitters::Vertical)),
                    '/' => Some(Mirror(Mirrors::Right)),
                    '\\' => Some(Mirror(Mirrors::Left)),
                    _ => None,
                } {
                    Some((IVec2::new(col as i32, row as i32), o))
                } else {
                    None
                }
            })
        })
        .collect();
    Map {
        objects,
        height: input.lines().count() as i32,
        width: input.lines().next().unwrap().len() as i32,
    }
}

fn simulate_beam(map: &Map, start: &Point, dir: Dir, energized: &mut HashSet<(Point, Dir)>) {
    let mut curr: Point = start.clone();
    let mut dir = dir;
    loop {
        let x: Point = dir.into();
        let next_point: Point = curr + x;
        if next_point.x >= map.width
            || next_point.x < 0
            || next_point.y >= map.height
            || next_point.y < 0
            || energized.contains(&(next_point, dir))
        {
            return;
        }

        energized.insert((next_point.clone(), dir));

        if let Some(obj) = map.objects.get(&next_point) {
            match obj {
                Splitter(Splitters::Vertical) => {
                    if [Dir::East, Dir::West].contains(&dir) {
                        simulate_beam(map, &next_point, Dir::North, energized);
                        simulate_beam(map, &next_point, Dir::South, energized);
                        return;
                    }
                }
                Splitter(Splitters::Horizontal) => {
                    if [Dir::North, Dir::South].contains(&dir) {
                        simulate_beam(map, &next_point, Dir::East, energized);
                        simulate_beam(map, &next_point, Dir::West, energized);
                        return;
                    }
                }
                Mirror(Mirrors::Left) => {
                    dir = match dir {
                        Dir::North => Dir::West,
                        Dir::South => Dir::East,
                        Dir::East => Dir::South,
                        Dir::West => Dir::North,
                    };
                }
                Mirror(Mirrors::Right) => {
                    dir = match dir {
                        Dir::North => Dir::East,
                        Dir::South => Dir::West,
                        Dir::East => Dir::North,
                        Dir::West => Dir::South,
                    };
                }
            }
        }
        curr = next_point;
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-16-2023.txt");
    // let input = include_str!("../../inputs/test-16-2023.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn count_energized(map: &Map, start: &Point, dir: Dir) -> usize{
    let mut energized = Default::default();
    simulate_beam(&map, start, dir, &mut energized);
    energized.iter().map(|(p, _)| p).unique().count()
}

fn part_one(input: &'static str) -> Result<usize> {
    let map = parse(input);
    let mut energized = Default::default();
    simulate_beam(&map, &Point::new(-1, 0), Dir::East, &mut energized);

    Ok(energized.iter().map(|(p, _)| p).unique().count())
}

fn part_two(input: &'static str) -> Result<usize> {
    let map = parse(input);
    let mut res = 0;
    for dir in [Dir::East, Dir::South, Dir::West, Dir::North] {
        for i in 0..map.height {
            let p = match dir {
                Dir::North => {Point::new(i, map.height)}
                Dir::South => {Point::new(i, -1)}
                Dir::East => {Point::new(-1, i)}
                Dir::West => {Point::new(map.width, i)}
            };
            res = max(res, count_energized(&map, &p, dir));
        }
    }

    Ok(res)
}
