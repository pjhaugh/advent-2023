use std::iter::zip;
use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-13-2023.txt");
    // let input = include_str!("../../inputs/test-13.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}

fn parse(input: &str) -> Vec<Vec<Vec<char>>> {
    input.split("\n\n").map(|pat|
        pat.lines().map(|line| {
            line.chars().collect()
        }).collect()
    ).collect()
}

fn verticals(pat: &Vec<Vec<char>>) -> usize {
    let cols = pat[0].len();
    (1..cols).filter(|index| {
        pat.iter().all(|line|
            zip(
                line[0..*index].iter().rev(),
                line[*index..cols].iter()
            ).all(|(a, b)| a==b))
    }).sum::<usize>()
}

fn horizontals(pat: &Vec<Vec<char>>) -> usize {
    (1..pat.len()).filter(|index|
        zip(
            pat[0..*index].iter().rev(),
            pat[*index..pat.len()].iter()
        ).all(|(a, b)| a==b)
    ).sum::<usize>()
}

fn vertical_smudge(pat: &Vec<Vec<char>>) -> usize {
    let cols = pat[0].len();
    for index in 1..cols {
        if pat.iter().map(|line|
            zip(
                line[0..index].iter().rev(),
                line[index..cols].iter()
            ).filter(|(a, b)| a!=b).count()).sum::<usize>() == 1 {
            return index;
        }
    }
    0
}

fn horizontal_smudge(pat: &Vec<Vec<char>>) -> usize {
    for index in 1..pat.len() {
        if zip(
            pat[0..index].iter().rev(),
            pat[index..pat.len()].iter()
        ).map(|(a, b)|
            zip(a.iter(), b.iter()).filter(|(x, y)| x != y).count()
        ).sum::<usize>() == 1 {
            return index;
        }
    }
    0
}


fn part_one(input: &'static str) -> Result<usize> {
    let patterns = parse(input);
    let verts = patterns.iter().map(verticals).sum::<usize>();
    let horz = patterns.iter().map(horizontals).sum::<usize>();
    Ok((horz*100) + verts)
}


fn part_two(input: &'static str) -> Result<usize> {
    let patterns = parse(input);
    let mut res = 0;
    for pat in &patterns {
        let h = horizontal_smudge(pat);
        if h > 0 {
            res += h * 100;
        } else {
            let v = vertical_smudge(pat);
            assert_ne!(0, v);
            res += v;
        }
    }
    Ok(res)
}
