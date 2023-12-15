use std::ops::Rem;
use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
struct Lens {
    label: &'static str,
    focal: u64,
}


fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-15-2023.txt");
    // let input = include_str!("../../inputs/test-15.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn hash(input: &str) -> u64 {
    input.chars().map(|c| c as u64).fold(0, |acc, x| { ((acc + x) * 17).rem(256) })
}

fn part_one(input: &'static str) -> Result<u64> {
    Ok(input.trim().split(",").map(hash).sum())
}


fn part_two(input: &'static str) -> Result<u64> {
    let mut hashmap: Vec<Vec<Lens>> = (0..256).map(|_| Vec::new()).collect();
    for inst in input.trim().split(",") {
        if inst.ends_with("-") {
            let label = inst.strip_suffix("-").unwrap();
            let h = hash(label);
            if let Some(v) = hashmap.get_mut(h as usize) {
                *v = v.iter().filter_map(|l| if l.label != label { Some(*l) } else { None }).collect()
            }
        } else {
            let (label, focal) = inst.trim().split("=").take(2).collect_tuple().unwrap();
            let focal = u64::from_str(focal)?;
            let h = hash(label) as usize;
            let v = hashmap.get_mut(h).unwrap();
            if let Some(l) = v.iter_mut().filter(|l| l.label == label).next() {
                l.focal = focal;
            } else {
                v.push(Lens { label, focal })
            }
        }

        // println!("After \"{inst}\"");
        // for (i, v) in hashmap.iter().enumerate().filter(|(i, v)| !v.is_empty()) {
        //     println!("Box {i}: {v:?}");
        // }
        // println!();
    }
    let res = hashmap.iter().enumerate().flat_map(|(num, v)|{
        v.iter().enumerate().map(move |(i, l)| (num + 1) * (i + 1) * (l.focal as usize))
    }).sum::<usize>() as u64;

    Ok(res)
}
