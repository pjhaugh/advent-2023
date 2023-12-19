use std::collections::HashMap;
use std::fmt::format;
use std::ops::RangeInclusive;
use std::thread::park_timeout_ms;

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{IResult, Parser};
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, anychar, one_of};
use nom::combinator::map;
use nom::multi::{many_till, separated_list1};
use nom::sequence::preceded;

use crate::Rule::IfJump;

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug, Copy, Clone)]
enum Property {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Property {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'x' => Ok(Property::X),
            'm' => Ok(Property::M),
            'a' => Ok(Property::A),
            's' => Ok(Property::S),
            _ => Err(anyhow!("Bad property"))
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Op {
    LT,
    GT,
    ANY,
}

#[derive(Debug, Copy, Clone)]
enum Rule {
    IfJump(Property, Op, u64, &'static str),
    A,
    R,
}

fn parse_rule(input: &'static str) -> IResult<&str, Rule> {
    match input {
        "A" => Ok(("", Rule::A)),
        "R" => Ok(("", Rule::R)),
        v if !v.contains(|char| ['<', '>'].contains(&char)) => {
            Ok(
                map(
                    alpha1,
                    |v|
                        IfJump(Property::A, Op::ANY, 0, v),
                )(v)?
            )
        }
        _ => {
            let (input, property) = map(one_of("xmas"),
                                        |s| Property::try_from(s).expect("property"),
            )(input)?;
            let (input, op) = map(one_of("<>"),
                                  |s| match s {
                                      '<' => Op::LT,
                                      '>' => Op::GT,
                                      _ => panic!("Bad op {s}")
                                  })(input)?;
            let (input, num) = nom::character::complete::u64(input)?;
            let (input, tag) = preceded(tag(":"), alpha1)(input)?;
            Ok((input, Rule::IfJump(property, op, num, tag)))
        }
    }
}

fn parse_map(input: &'static str) -> Result<HashMap<&str, Vec<Rule>>> {
    input.lines().map(|line| {
        let (line, key) = take_until::<_, _, nom::error::Error<_>>("{")(line)?;
        let (line, values) = preceded(
            tag("{"),
            separated_list1(
                tag(","),
                parse_rule,
            ),
        )(line)?;
        Ok((key, values))
    }).collect::<Result<HashMap<_, _>>>()
}

fn parse_parts(input: &'static str) -> Result<Vec<Part>> {
    let mut p = many_till(anychar::<_, nom::error::Error<_>>, nom::character::complete::u64);
    input.lines().map(|line| {
        let (line, (_, x)) = p.parse(line)?;
        let (line, (_, m)) = p.parse(line)?;
        let (line, (_, a)) = p.parse(line)?;
        let (_, (_, s)) = p.parse(line)?;
        Ok(Part { x, m, a, s })
    }).collect::<Result<Vec<_>>>()
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-19-2023.txt");
    // let input = include_str!("../../inputs/test-19-2023.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}


fn part_one(input: &'static str) -> Result<u64> {
    let start = "in";
    let rules_text = input.split("\n\n").next().unwrap();
    let parts_text = input.split("\n\n").last().unwrap();
    let rules_map = parse_map(rules_text)?;
    let parts = parse_parts(parts_text)?;

    for (key, rules) in rules_map.iter() {
        println!("\"{key}\":  {rules:?}");
    }

    let mut res = 0;


    'part: for part in &parts {
        println!("Part: {part:?}");
        let mut rules = rules_map.get(start).expect("bad link");
        let mut index = 0;
        while let Rule::IfJump(prop, op, num, dest) = rules[index] {
            let val = match prop {
                Property::X => { part.x }
                Property::M => { part.m }
                Property::A => { part.a }
                Property::S => { part.s }
            };
            if match op {
                Op::LT => { val < num }
                Op::GT => { val > num }
                Op::ANY => { true }
            } {
                match dest {
                    "A" => {
                        println!("Accepted part {part:?}");
                        res += part.x + part.m + part.a + part.s;
                        continue 'part;
                    }
                    "R" => { continue 'part; }
                    v => {
                        println!("Jumping to {v}");
                        rules = rules_map.get(v).unwrap();
                        index = 0;
                    }
                }
            } else {
                index += 1;
            }
        }
        match rules[index] {
            Rule::IfJump(_, _, _, _) => { unreachable!("Bad loop exit") }
            Rule::A => {
                println!("Accepted part {part:?}");
                res += part.x + part.m + part.a + part.s;
            }
            Rule::R => { continue; }
        }
    }

    Ok(res)
}

fn count_accepted(rules_map: &HashMap<&str, Vec<Rule>>, workflow: &str, index: usize, x: RangeInclusive<u64>, m: RangeInclusive<u64>, a: RangeInclusive<u64>, s: RangeInclusive<u64>) -> u64 {
    if workflow == "A" {
        return (x.try_len().expect("Should not be non-zero")
            * m.try_len().expect("Should not be non-zero")
            * a.try_len().expect("Should not be non-zero")
            * s.try_len().expect("Should not be non-zero")) as u64;
    }
    if workflow == "R" {
        return 0;
    }
    let rule = rules_map.get(workflow).expect(workflow).get(index).unwrap();

    match rule {
        IfJump(prop, op, num, dest) => {
            match op {
                Op::ANY => {
                    match *dest {
                        "A" => {
                            (x.try_len().expect("Should not be non-zero")
                                * m.try_len().expect("Should not be non-zero")
                                * a.try_len().expect("Should not be non-zero")
                                * s.try_len().expect("Should not be non-zero")) as u64
                        },
                        "R" => {0},
                        v => { count_accepted(rules_map, v, 0, x, m, a, s) }
                    }
                },
                op => {
                    let (lower, upper) = match prop {
                        Property::X => {x.clone()}
                        Property::M => {m.clone()}
                        Property::A => {a.clone()}
                        Property::S => {s.clone()}
                    }.into_inner();
                    let lesser_upper = match op {
                        Op::LT => {num-1}
                        Op::GT => {*num}
                        Op::ANY => {unreachable!("")}
                    };
                    let r1 = lower..=lesser_upper;
                    let r2 = lesser_upper+1..=upper;

                    let ((w1, i1), (w2, i2)) = if op == &Op::LT {
                        ((*dest, 0), (workflow, index + 1))
                    } else {
                        ((workflow, index + 1), (*dest, 0))
                    };

                    match prop {
                        Property::X => {
                            count_accepted(rules_map, w1, i1, r1, m.clone(), a.clone(), s.clone())
                            + count_accepted(rules_map, w2, i2, r2, m, a, s)
                        }
                        Property::M => {
                            count_accepted(rules_map, w1, i1, x.clone(), r1, a.clone(), s.clone())
                                + count_accepted(rules_map, w2, i2, x, r2, a, s)
                        }
                        Property::A => {
                            count_accepted(rules_map, w1, i1, x.clone(), m.clone(), r1, s.clone())
                                + count_accepted(rules_map, w2, i2, x, m.clone(), r2, s)
                        }
                        Property::S => {
                            count_accepted(rules_map, w1, i1, x.clone(), m.clone(), a.clone(), r1)
                                + count_accepted(rules_map, w2, i2, x, m.clone(), a.clone(), r2)
                        }
                    }
                }
            }
        }
        Rule::A => {
            (x.try_len().expect("Should not be non-zero")
            * m.try_len().expect("Should not be non-zero")
                * a.try_len().expect("Should not be non-zero")
                * s.try_len().expect("Should not be non-zero")).try_into().unwrap()
        }
        Rule::R => { 0 }
    }

}



fn part_two(input: &'static str) -> Result<u64> {
    let start = "in";
    let rules_text = input.split("\n\n").next().unwrap();
    let rules_map = parse_map(rules_text)?;

    Ok(count_accepted(&rules_map, start, 0, 1..=4000, 1..=4000, 1..=4000, 1..=4000))
}
