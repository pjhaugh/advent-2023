use anyhow::Result;
use itertools::{Itertools, join};

type Point = (usize, usize);

fn main() -> Result<()> {
    let input = include_str!("../../inputs/input-11-2023.txt");
    // let input = include_str!("../../inputs/test-11.txt");

    let part_1_ans = part_one(input)?;
    println!("Part One answer: {part_1_ans}");

    let part_2_ans = part_two(input)?;
    println!("Part Two answer: {part_2_ans}");

    Ok(())
}


fn part_one(input: &'static str) -> Result<usize> {
    let lines = input.lines().collect_vec();
    let empty_cols = (0..lines[0].len()).filter(|i|
        lines.iter().all(|l|
            l.as_bytes().get(*i).unwrap() == &b'.')).collect_vec();
    let empty_rows = lines.iter().enumerate().filter_map(|(i, line)|
        if line.chars().all(|c| c == '.') {
            Some (i)
        } else {
            None
        }
    ).collect_vec();
    let new_width = lines[0].len() + empty_cols.len();
    let input = join(
        lines.iter().enumerate().flat_map(|(row, l)|{
            let mut chars = Vec::new();
            chars.append(&mut l.chars().enumerate().flat_map(|(col, c)| {
                if empty_cols.contains(&col) {
                    Vec::from(['.', '.'])
                } else {
                    Vec::from([c])
                }
            }).collect_vec());
            if empty_rows.contains(&row) {
                Vec::from([chars.iter().join(""), (0..new_width).map(|_| ".").collect()])
            } else {
                Vec::from([chars.iter().join("")])
            }
        }),
        "\n"
    );
    let galaxies = input.lines().enumerate().flat_map(|(row, line)| {
        line.chars().enumerate().filter_map(move |(col, c)|
            if c == '#' { Some((row, col)) } else { None }
        )
    }).collect::<Vec<Point>>();

    Ok(galaxies.iter().tuple_combinations::<(_, _)>().map(|(a, b)|{
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    }).sum::<usize>())
}

fn expansion(empties: &Vec<usize>, a: usize, b: usize) -> usize {
    let range = if a < b { a..b } else { b..a };
    let factor = 1_000_000 - 1;
    range.filter(|x| empties.contains(x)).count() * factor
}

fn part_two(input: &'static str) -> Result<usize> {
    let lines = input.lines().collect_vec();
    let empty_cols = (0..lines[0].len()).filter(|i|
        lines.iter().all(|l|
            l.as_bytes().get(*i).unwrap() == &b'.')).collect_vec();
    let empty_rows = lines.iter().enumerate().filter_map(|(i, line)|
        if line.chars().all(|c| c == '.') {
            Some (i)
        } else {
            None
        }
    ).collect_vec();
    let galaxies = input.lines().enumerate().flat_map(|(row, line)| {
        line.chars().enumerate().filter_map(move |(col, c)|
            if c == '#' { Some((row, col)) } else { None }
        )
    }).collect::<Vec<Point>>();
    Ok(galaxies.iter().tuple_combinations::<(_, _)>().map(|(a, b)|{
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1) + expansion(&empty_rows, a.0, b.0) + expansion(&empty_cols, a.1, b.1)
    }).sum::<usize>())
}
