use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{bail, Result};
use glam::IVec2;
use itertools::Itertools;
use priority_queue::PriorityQueue;

type Point = IVec2;

const NORTH: IVec2 = IVec2::new(0, -1);
const SOUTH: IVec2 = IVec2::new(0, 1);
const EAST: IVec2 = IVec2::new(1, 0);
const WEST: IVec2 = IVec2::new(-1, 0);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl From<Dir> for Point {
    fn from(value: Dir) -> Self {
        match value {
            Dir::North => NORTH,
            Dir::South => SOUTH,
            Dir::East => EAST,
            Dir::West => WEST,
        }
    }
}

impl Add<Point> for Dir {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::from(self) + rhs
    }
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-17-2023.txt");
    let input = include_str!("../../inputs/test-17-2023.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<i32>>,
    height: i32,
    width: i32,
}

impl Map {
    fn contains(self: &Self, point: &Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width && point.y < self.height
    }

    fn access(self: &Self, point: &Point) -> Option<i32> {
        Some(*self.grid.get(point.y as usize)?.get(point.x as usize)?)
    }
}

fn manhattan(p1: &Point, p2: &Point) -> i32 {
    (p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)) as i32
}


#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Step {
    point: Point,
    dir: Dir,
    steps: i32,
}



fn find_least(map: &Map, start: Point, goal: Point, min_steps: i32, max_steps: i32, can_stop: bool) -> Result<i32> {
    let mut open_set: PriorityQueue<Step, i32> =
        PriorityQueue::from(vec![
            (Step {
                point: start + EAST,
                dir: Dir::East,
                steps: 1,
            }, -1 * map.access(&Point::from(Dir::East)).unwrap()),
            (Step {
                point: start + SOUTH,
                dir: Dir::South,
                steps: 1,
            }, -1 * map.access(&Point::from(Dir::South)).unwrap()),
        ]);

    let mut came_from: HashMap<Step, Step> = Default::default();

    let mut g_score: HashMap<Step, i32> = HashMap::from_iter(vec![
        (Step {
            point: start + EAST,
            dir: Dir::East,
            steps: 1,
        }, map.access(&Point::from(Dir::East)).unwrap()),
        (Step {
            point: start + SOUTH,
            dir: Dir::South,
            steps: 1,
        }, map.access(&Point::from(Dir::South)).unwrap()),
    ]);

    let mut f_score: HashMap<Step, i32> = HashMap::from_iter(vec![
        (Step {
            point: start + EAST,
            dir: Dir::East,
            steps: 1,
        }, map.access(&EAST).unwrap() + manhattan(&EAST, &goal)),
        (Step {
            point: start + SOUTH,
            dir: Dir::South,
            steps: 1,
        }, map.access(&SOUTH).unwrap() + manhattan(&SOUTH, &goal)),
    ]);

    while !open_set.is_empty() {
        let (curr, weight) = open_set.pop().unwrap();
        if curr.point == goal {
            return Ok(-1 * weight);
        }
        let mut neighbors = Vec::new();
        if curr.steps < max_steps && map.contains(&(curr.dir + curr.point)) {
            neighbors.push(Step {
                point: curr.dir + curr.point,
                dir: curr.dir,
                steps: curr.steps + 1,
            })
        }
        for turn in match curr.dir {
            Dir::North => { [Dir::East, Dir::West] }
            Dir::South => { [Dir::East, Dir::West] }
            Dir::East => { [Dir::North, Dir::South] }
            Dir::West => { [Dir::North, Dir::South] }
        } {
            if curr.steps >= min_steps && map.contains(&(turn + curr.point)) {
                neighbors.push(Step {
                    point: turn + curr.point,
                    dir: turn,
                    steps: 1,
                })
            }
        }

        for neighbor in neighbors {
            if !can_stop && neighbor.point == goal && neighbor.steps < min_steps {
                continue;
            }
            let tentative_g_score = g_score.get(&curr).unwrap() + map.access(&neighbor.point).unwrap();
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor.clone(), curr.clone());
                g_score.insert(neighbor.clone(), tentative_g_score);
                f_score.insert(neighbor.clone(), tentative_g_score + manhattan(&neighbor.point, &goal));
                if let Some((_, priority)) = open_set.get(&neighbor) {
                    if (-1 * tentative_g_score) < *priority {
                        open_set.change_priority(&neighbor, -1 * tentative_g_score);
                    }
                } else {
                    open_set.push(neighbor.clone(), -1 * tentative_g_score);
                }
            }
        }
    }

    bail!("No path")
}

fn part_one(input: &'static str) -> Result<i32> {
    let grid: Vec<Vec<i32>> = input.lines().map(|line| line.chars().map(|c| i32::from_str(c.to_string().as_str()).expect("parse")).collect()).collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let map = Map { grid, height, width };

    let res = find_least(&map, Point::new(0, 0), Point::new(map.width - 1, map.height - 1), 0, 3, true)?;

    Ok(res)
}


fn part_two(input: &'static str) -> Result<i32> {
    let grid: Vec<Vec<i32>> = input.lines().map(|line| line.chars().map(|c| i32::from_str(c.to_string().as_str()).expect("parse")).collect()).collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let map = Map { grid, height, width };

    let res = find_least(&map, Point::new(0, 0), Point::new(map.width - 1, map.height - 1), 4, 10, false)?;

    Ok(res)
}
