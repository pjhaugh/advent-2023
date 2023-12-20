use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use anyhow::Result;
use itertools::rev;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::sequence::preceded;

use crate::Module::{Broadcast, Conjunction, Flip};

mod utils;

#[derive(Debug, Clone)]
enum Module {
    Flip(bool, Vec<&'static str>),
    Conjunction(HashMap<&'static str, bool>, Vec<&'static str>),
    Broadcast(Vec<&'static str>),
}

fn parse_modules(input: &'static str) -> Result<HashMap<&str, Module>> {
    let mut map: HashMap<&'static str, Module> = Default::default();
    let mut connections: Vec<(&'static str, &'static str)> = Default::default();
    for line in input.lines() {
        let (line, name) = take_till::<_, _, nom::error::Error<_>>(|s| s == ' ')(line)?;
        let (_, dests) = preceded(
            tag::<_, _, nom::error::Error<_>>(" -> "),
            separated_list1(
                tag(", "),
                alpha1,
            ),
        )(line)?;
        let (tag, module) = match &name.chars().next().unwrap() {
            '%' => { (&name[1..name.len()], Flip(false, dests.clone())) }
            '&' => {
                (&name[1..name.len()], Conjunction(HashMap::new(), dests.clone()))
            }
            'b' => { (name, Broadcast(dests.clone())) }
            _ => { unreachable!("Bad module") }
        };
        map.insert(tag, module);
        dests.iter().for_each(|x| connections.push((tag, x)));
    }
    println!("{map:?}");
    for (send, receive) in &connections {
        if !map.contains_key(receive) {
            println!("{send} {receive}");
        }
        if let Some(Conjunction(memory, _ )) = map.get_mut(receive) {
            memory.insert(send, false);
        }
    }

    Ok(map.iter().map(|(name, m)| (*name, m.clone())).collect())
}

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-20-2023.txt");
    // let input = include_str!("../../inputs/test-20-2023.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn process_signal(modules: &mut HashMap<&str, Module>,
                  queue: &mut VecDeque<(&'static str, (&'static str, bool))>,
                  num_low: &mut u64,
                  num_high: &mut u64,
                  cycles: &mut HashMap<&str, u64>,
                  count: &u64
) {
    if let Some((name, (upstream, signal))) = queue.pop_front() {
        // println!("{upstream} -{}-> {name}", if signal {"high"} else {"low"});
        if let Some(module) = modules.get_mut(name) {

            match module {
                Flip(state, dests) => {
                    match signal {
                        true => {},
                        false => {
                            *state = !*state;
                            dests.iter().for_each(|d| {
                                let output = *state;
                                queue.push_back((d, (name, output)));
                                if output {
                                    *num_high += 1;
                                } else {
                                    *num_low += 1;
                                }
                            })
                        }
                    }
                }
                Conjunction(inputs, dests) => {
                    inputs.insert(upstream, signal);
                    let output = !inputs.iter().all(|(_, b)| *b);
                    dests.iter().for_each(|d| {
                        queue.push_back((d, (name, output)));
                        if output {
                            cycles.iter_mut().for_each(|(k, v)|
                                if k == &name {
                                    *v = *count;
                                }
                            );
                            *num_high += 1;
                        } else {
                            *num_low += 1;
                        }
                    })
                }
                Broadcast(dests) => {
                    dests.iter().for_each(|d| {
                        queue.push_back((d, (name, signal)));
                        if signal {
                            *num_high += 1;
                        } else {
                            *num_low += 1;
                        }
                    })
                }
            }


        }
    } else {
        panic!("Called on empty queue");
    }
}

fn part_one(input: &'static str) -> Result<u64> {
    let mut mods = parse_modules(input)?;
    let mut num_low = 0;
    let mut num_high = 0;
    let mut count = 0;
    let mut finished = false;

    let mut queue: VecDeque<(&'static str, (&'static str, bool))> = Default::default();
    let mut cycles = HashMap::from([("nl", 0), ("lr", 0), ("gt", 0), ("vr", 0)]);

    for _ in 0..1000 {
        queue.push_back(("broadcaster", ("button", false)));
        num_low += 1;
        while !queue.is_empty() {
            process_signal(&mut mods, &mut queue,
                            &mut num_low, &mut num_high, &mut cycles, &count);
        }
    }

    Ok(num_low * num_high)
}


fn part_two(input: &'static str) -> Result<u64> {
    let mut mods = parse_modules(input)?;
    let mut num_low = 0;
    let mut num_high = 0;
    let mut button_count = 0;

    let mut queue: VecDeque<(&'static str, (&'static str, bool))> = Default::default();

    let mut cycles = HashMap::from([("nl", 0), ("lr", 0), ("gt", 0), ("vr", 0)]);


    loop {
        queue.push_back(("broadcaster", ("button", false)));
        num_low += 1;
        button_count += 1;
        while !queue.is_empty() {
            process_signal(&mut mods, &mut queue, &mut num_low, &mut num_high, &mut cycles, &button_count);
        }
        if cycles.values().all(|x| x != &0) {
            break;
        }
    }
    Ok(utils::utils::lcm(cycles.values().map(|u| *u).collect()))
}
