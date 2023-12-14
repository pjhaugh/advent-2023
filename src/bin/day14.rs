use std::collections::{BTreeSet, HashMap};
use std::ops::{Add, Rem};

use anyhow::Result;
use glam::I64Vec2;
use itertools::Itertools;

use crate::Rock::{Round, Square};

#[derive(Debug, Eq, PartialEq)]
enum Rock {
    Round,
    Square,
}

type Point = I64Vec2;

struct Map {
    rocks: HashMap<Point, Rock>,
    height: i64,
    width: i64,
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-14-2023.txt");
    // let input = include_str!("../../inputs/test-14.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn parse(input: &str) -> Map {
    let rocks = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(col, c)| ['O', '#'].contains(c))
                .map(move |(col, c)| {
                    (
                        I64Vec2::new(col as i64, row as i64),
                        match c {
                            'O' => Round,
                            '#' => Square,
                            _ => panic!("Bad input {c} at {row} {col}"),
                        },
                    )
                })
        })
        .collect();
    let height = input.lines().count() as i64;
    let width = input.lines().next().unwrap().len() as i64;
    Map {
        rocks,
        height,
        width,
    }
}

fn roll(map: &mut Map, dir: &I64Vec2) {
    loop {
        let mut to_del: Vec<Point> = Default::default();

        for (point, rock) in map.rocks.iter() {
            let mut p2 = point.add(*dir);
            while !map.rocks.contains_key(&p2)
                && *rock != Square
                && p2.x >= 0
                && p2.x < map.width
                && p2.y >= 0
                && p2.y < map.height
            {
                to_del.push(point.clone());
                p2 = p2.add(*dir);
            }
        }

        if to_del.is_empty() {
            return;
        }

        for p in &to_del {
            map.rocks.remove(p);
            map.rocks.insert(p.add(*dir), Round);
        }
    }
}

fn print_map(map: &Map) {
    for row in 0..map.height {
        let line = (0..map.width)
            .map(|col| match map.rocks.get(&I64Vec2::new(col, row)) {
                None => ".",
                Some(Round) => "O",
                Some(Square) => "#",
            })
            .join("");
        println!("{}", line);
    }
}

fn load(map: &Map) -> i64 {
    map.rocks
        .iter()
        .filter_map(|(p, r)| match r {
            Round => Some(map.height - p.y),
            Square => None,
        })
        .sum::<i64>()
}

fn part_one(input: &'static str) -> Result<i64> {
    let mut map = parse(input);
    const NORTH: I64Vec2 = I64Vec2::new(0, -1);
    roll(&mut map, &NORTH);
    print_map(&map);
    Ok(load(&map))
}

fn part_two(input: &'static str) -> Result<i64> {
    let mut map = parse(input);
    const NORTH: I64Vec2 = I64Vec2::new(0, -1);
    const SOUTH: I64Vec2 = I64Vec2::new(0, 1);
    const EAST: I64Vec2 = I64Vec2::new(1, 0);
    const WEST: I64Vec2 = I64Vec2::new(-1, 0);

    let mut hist: HashMap<BTreeSet<(i64, i64)>, usize> = Default::default();
    let mut loads: Vec<_> = Default::default();
    let mut index: Option<usize> = None;

    for i in 0..1_000_000_000 {
        roll(&mut map, &NORTH);
        roll(&mut map, &WEST);
        roll(&mut map, &SOUTH);
        roll(&mut map, &EAST);
        let state = map
            .rocks
            .iter()
            .filter_map(|(p, r)| match r {
                Round => Some((p.x, p.y)),
                _ => None,
            })
            .collect();
        if let Some(val) = hist.get(&state) {
            index = Some(*val);
            break;
        }
        hist.insert(state, i);
        loads.push(load(&map));
    }

    let rem = 1_000_000_000 - (index.unwrap() + 1);

    let slice = &loads[index.unwrap()..loads.len()];

    let index = rem.rem(slice.len());

    println!("{}", index);

    Ok(slice[index])
}
