use std::collections::HashSet;

use anyhow::{bail, Result};

use crate::Pipe::{GROUND, START};

type Point = (usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    START,
    GROUND,
    X,
}

impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        use Pipe::*;
        Ok(match value {
            '|' => NS,
            '-' => EW,
            'L' => NE,
            'J' => NW,
            '7' => SW,
            'F' => SE,
            '.' => GROUND,
            'S' => START,
            _ => bail!("Bad input {value}"),
        })
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-10-2023.txt");
    // let input = include_str!("../../inputs/test-10.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn find_adjacent(pipes: &Vec<Vec<Pipe>>, point: &Point) -> (Point, Point) {
    let pipe = find_beneath(pipes, point);
    let (row, col) = *point;
    match pipe {
        Pipe::NS => ((row - 1, col), (row + 1, col)),
        Pipe::EW => ((row, col - 1), (row, col + 1)),
        Pipe::NE => ((row - 1, col), (row, col + 1)),
        Pipe::NW => ((row - 1, col), (row, col - 1)),
        Pipe::SW => ((row + 1, col), (row, col - 1)),
        Pipe::SE => ((row + 1, col), (row, col + 1)),
        _ => panic!("Bad adjacent {row} {col}"),
    }
}

fn find_start(pipes: &Vec<Vec<Pipe>>) -> Point {
    pipes
        .iter()
        .enumerate()
        .flat_map(|(r, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(c, p)| if p == &START { Some((r, c)) } else { None })
        })
        .next()
        .expect("Could not locate start")
}

fn find_beneath(pipes: &Vec<Vec<Pipe>>, loc: &Point) -> Pipe {
    use Pipe::*;
    let (row, col) = *loc;
    if pipes[row][col] != START {
        return pipes[row][col];
    }
    let north = if let Some(row) = row.checked_sub(1) {
        pipes.get(row).and_then(|r| r.get(col))
    } else {
        None
    };
    let south = if let Some(row) = row.checked_add(1) {
        pipes.get(row).and_then(|r| r.get(col))
    } else {
        None
    };
    let west = if let Some(col) = col.checked_sub(1) {
        pipes.get(row).and_then(|r| r.get(col))
    } else {
        None
    };
    let east = if let Some(col) = col.checked_add(1) {
        pipes.get(row).and_then(|r| r.get(col))
    } else {
        None
    };
    if north.is_some() && [SE, SW, NS].contains(north.unwrap()) {
        if east.is_some() && [EW, NW, SW].contains(east.unwrap()) {
            return NE;
        }
        if west.is_some() && [NE, EW, SE].contains(west.unwrap()) {
            return NW;
        }
        return NS;
    }
    if south.is_some() && [NE, NW, NS].contains(south.unwrap()) {
        if east.is_some() && [EW, NW, SW].contains(east.unwrap()) {
            return SE;
        }
        if west.is_some() && [NE, EW, SE].contains(west.unwrap()) {
            return SW;
        }
    }
    EW
}

fn advance(pipes: &Vec<Vec<Pipe>>, curr: &Point, hist: &Point) -> (Point, Point) {
    let (a, b) = find_adjacent(pipes, curr);
    if hist == &a {
        (b, *curr)
    } else {
        (a, *curr)
    }
}

fn part_one(input: &'static str) -> Result<usize> {
    let pipes: Vec<Vec<Pipe>> = input
        .lines()
        .map(|l| l.chars().map(Pipe::try_from).collect::<Result<Vec<_>>>())
        .collect::<Result<Vec<Vec<_>>>>()?;

    let start = find_start(&pipes);

    let (mut ptr_one, mut ptr_two) = find_adjacent(&pipes, &start);
    let (mut ptr_one_hist, mut ptr_two_hist) = (start, start);

    let mut res = 1;

    loop {
        res += 1;
        (ptr_one, ptr_one_hist) = advance(&pipes, &ptr_one, &ptr_one_hist);
        (ptr_two, ptr_two_hist) = advance(&pipes, &ptr_two, &ptr_two_hist);

        if ptr_one == ptr_two {
            return Ok(res);
        }
    }
}

fn part_two(input: &'static str) -> Result<usize> {
    let mut res = 0;
    let mut pipes: Vec<Vec<Pipe>> = input
        .lines()
        .map(|l| l.chars().map(Pipe::try_from).collect::<Result<Vec<_>>>())
        .collect::<Result<Vec<Vec<_>>>>()?;

    let start = find_start(&pipes);

    let (mut ptr_one, mut ptr_two) = find_adjacent(&pipes, &start);
    let (mut ptr_one_hist, mut ptr_two_hist) = (start, start);
    let mut points: HashSet<Point> = HashSet::from([start, ptr_one, ptr_two]);

    loop {
        (ptr_one, ptr_one_hist) = advance(&pipes, &ptr_one, &ptr_one_hist);
        (ptr_two, ptr_two_hist) = advance(&pipes, &ptr_two, &ptr_two_hist);
        points.insert(ptr_one);
        points.insert(ptr_two);
        if ptr_one == ptr_two {
            break;
        }
    }

    let cols = pipes[0].len();

    for row in 0..pipes.len() {
        for col in 0..cols {
            if !points.contains(&(row, col)) {
                pipes[row][col] = GROUND;
            }
        }
    }

    for (row, line) in pipes.clone().iter().enumerate() {
        let mut in_loop = false;
        for (col, pipe) in line.iter().enumerate() {
            use Pipe::*;
            if [NS, NE, NW].contains(pipe) {
                in_loop = !in_loop;
            } else if pipe == &GROUND && in_loop {
                res += 1;
                pipes[row][col] = X;
            }
        }
    }

    for line in &pipes {
        println!(
            "{}",
            line.iter()
                .map(|p| match p {
                    Pipe::NS => "|",
                    Pipe::EW => "-",
                    Pipe::NE => "L",
                    Pipe::NW => "J",
                    Pipe::SW => "7",
                    Pipe::SE => "F",
                    START => "S",
                    GROUND => ".",
                    Pipe::X => "X",
                })
                .collect::<String>()
        )
    }

    Ok(res)
}
